---
title: "Optimizing a Rust GPU matmul kernel for 1TFLOP+ performance"
authors: ["LegNeato"]
slug: optimizing-matmul
tags: ["demo", "code", "performance"]
draft: true
---

I read the excellent post [Optimizing a WebGPU Matmul Kernel for 1TFLOP+
Performance](https://www.nuss-and-bolts.com/p/optimizing-a-webgpu-matmul-kernel) by
[Zach Nussbaum](https://x.com/zach_nussbaum) and thought it might be fun to reimplement
it with [Rust GPU](https://Rust-gpu.github.io/).

We'll follow Zach's original post closely, comparing and contrasting using Rust vs the
WGSL and Typescript from his post.

At the end, I'll show some additional fun things that are enabled by using Rust on the
GPU.

<!-- truncate -->

A big thank you to [Zach](https://x.com/zach_nussbaum) for allowing me to reimplement
his blog post!

:::tip

The complete runnable code can be [found on
GitHub](https://github.com/Rust-GPU/rust-gpu.github.io/tree/main/blog/2024-11-21-optimizing-matrix-mul/code).

:::

## What is Rust GPU?

[Rust GPU](https://Rust-gpu.github.io/) is a project that allows you to write code for
GPUs using the Rust programming language. GPUs are typically programmed using
specialized languages like [WGSL](https://www.w3.org/TR/WGSL/),
[GLSL](https://developer.mozilla.org/en-US/docs/Games/Techniques/3D_on_the_web/GLSL_Shaders),
[MSL](https://developer.apple.com/documentation/metal/performing_calculations_on_a_gpu),
or
[HLSL](https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl).
Rust GPU changes this by letting you use Rust to write GPU programs (often called
"shaders" or "kernels").

These Rust GPU programs are then compiled into [SPIR-V](https://www.khronos.org/spir/),
a low-level format that [most GPUs understand](https://vulkan.gpuinfo.org/). Since
SPIR-V is the format [Vulkan](https://www.vulkan.org/) uses, Rust GPU makes it possible
to integrate Rust-based GPU programs into any Vulkan-compatible workflow.

For more details, check out the [Rust GPU website](https://Rust-gpu.github.io/) or the [GitHub repository](https://github.com/Rust-gpu/Rust-gpu).

## How does Rust GPU work?

Rust GPU focuses purely on compiling your Rust code into SPIR-V. This compiled code is
what the GPU executes. However, Rust GPU doesn't dictate how you handle CPU-to-GPU
communication or data transfer. You're free to choose a host CPU library written in
whatever language that fits your project. Some popular options in Rust include:

- **[ash](https://github.com/ash-rs/ash)**: Low-level Vulkan bindings for Rust, providing maximum control over Vulkan operations.
- **[vulkano](https://github.com/vulkano-rs/vulkano)**: A higher-level Vulkan library that simplifies common tasks.
- **[wgpu](https://github.com/gfx-rs/wgpu)**: A cross-platform library that abstracts GPU operations across Vulkan, DirectX, Metal, and WebGPU.

But again, you don't _have_ to use Rust for the CPU-side when using Rust on the GPU—any
language will do.

## What will we use?

In Zach's post, he writes his GPU programs in [WGSL](https://www.w3.org/TR/WGSL/). These
programs and their data are sent to and from the GPU via Typescript which talks to the
[WebGPU](https://en.wikipedia.org/wiki/WebGPU) CPU code built into the browser.

We'll take a different approach: writing GPU programs in Rust via Rust GPU and managing
everything—including the CPU-side code—in Rust. This means both the GPU programs and the
code controlling them will be written in the same language. If you are familiar with web
programming, what we are doing is conceptually similar to Javascript running on both the
server and the client.

Using Rust for both CPU and GPU has advantages, like consistent tooling and shared code.
But it also means we need to be clear about which code runs where. I've tried to make
sure this distinction is easy to follow.

To handle communication between our code on the CPU and GPU, we'll use
[`wgpu`](https://github.com/gfx-rs/wgpu). `wgpu` is a high-level Rust library that
implements the WebGPU API. On the web, it works directly with the browser's WebGPU
implementation. On native platforms, it translates API calls to the platform's GPU API
(Vulkan, DirectX, or Metal). This lets us run the same code on a wide range of
platforms, including Windows, Linux, macOS, iOS[^1], Android, and the web[^2].

By using Rust GPU and `wgpu`, we have a clean, portable setup with everything written in
Rust.

## GPU program basics

The smallest unit of execution is a thread, which executes the GPU program.

Workgroups are groups of threads: they are grouped together and run in parallel (they’re
called [thread blocks in
CUDA](<https://en.wikipedia.org/wiki/Thread_block_(CUDA_programming)>)). They can access
the same shared memory.

We can dispatch many of these workgroups at once. CUDA calls this a grid (which is made
of thread blocks).

Workgroups and dispatching workgroups are defined in 3D. The size of a workgroup is
defined by `compute(threads((x, y, z)))` where the number of threads per workgroup is
x \* y \* z.

## Writing the kernel

### Kernel 1: Naive kernel

The simplest way to compute a dot product between matrix A and B and write
to matrix C is for each row in A (of shape M), iterate over the columns of A (of shape
K) and multiply by the corresponding value of B.

Here, we have our first difference from Zach's post. In WGSL, you must define inputs at
the top-level scope:

import { WebGpuInputs } from './snippets/naive.tsx';

<WebGpuInputs/>

And then write your kernel:

import { WebGpuKernel } from './snippets/naive.tsx';

<WebGpuKernel/>

With Rust GPU, we specify the inputs as arguments to the kernel and configure them with
[procedural macros](https://doc.Rust-lang.org/reference/procedural-macros.html):

import { RustNaiveInputs } from './snippets/naive.tsx';

<RustNaiveInputs/>

This code looks like normal Rust code but _runs entirely on the GPU._

There are a couple of things to note about the Rust implementation:

1. The kernel uses the regular Rust
   [`#![no_std]`](https://www.reddit.com/r/Rust/comments/9eyc21/noob_what_exactly_is_no_std_and_why_is_it_so/)
   attribute, which is required because GPUs do not have access to Rust's standard
   library (`std`). Instead, you rely on `core` and `spirv_std` to provide `std`-like
   functionality.
2. Libraries are imported via `use`. The module system works exactly the same as regular Rust.
3. We're importing a vendored copy of [`glam`](https://github.com/bitshifter/glam-rs).
   This is the exact `glam` crate from [crates.io](https://crates.io/crates/glam).
4. The inner loop (`for i in 0..dimensions.k`) uses Rust's `for` syntax with a range.
   This is a higher-level abstraction compared to manually iterating with an index in
   other shader languages like WGSL, GLSL, or HLSL.
5. Read-only inputs are immutable references (`&Dimensions` / `&[f32]`) and writeable outputs are
   mutable references (`&mut [f32]`). This feels very familiar to anyone used to writing
   Rust.

#### What's with all the `usize`?

Rust defines `usize` as [the native pointer width of the hardware the code is running
on](https://doc.rust-lang.org/std/primitive.usize.html). This is important because Rust
uses `usize` for indexing slices to ensure that access is properly pointer-aligned.

On most GPU hardware, `usize` is effectively equivalent to `u32`. But the Rust compiler
doesn't assume that. It can't, because doing so could introduce problems—like if you ran
this code on hardware where `usize` is actually `u64`. Rust won't let you implicitly
treat a `u32` as a `usize`. You have to explicitly cast it, essentially telling the
compiler "I know this is safe for my target hardware."

This explicitness might seem tedious but it is one of the ways Rust prevents subtle
bugs. It forces you to think about whether your assumptions about hardware alignment and
pointer sizes are correct, making your code more portable and reliable.

:::info

Matrix multiplication is a pathological case with lots of indexing and row and column
calculations. Most Rust GPU code does not have nearly as many `usize` casts as these
examples.

:::

#### Dispatching workgroups

Each workgroup, since it’s only one thread (`#[spirv(compute(threads(1)))]`), processes
one `result[i, j]`.

To calculate the full matrix, we need to launch as many entries as there are in the
matrix. Here we specify that (`Uvec3::new(m * n, 1, 1`) on the CPU:

import { RustNaiveWorkgroupCount } from './snippets/naive.tsx';

<RustNaiveWorkgroupCount/>

The `dispatch_count()` function runs on the CPU and is used by the CPU-to-GPU API (in
our case `wgpu`) to configure and dispatch work to the GPU:

import { RustNaiveDispatch } from './snippets/naive.tsx';

<RustNaiveDispatch />

:::warning

This code appears more complicated than it needs to be. I abstracted the CPU-side code
that talks to the GPU using generics and traits so I could easily slot in different
kernels and their settings while writing the blog post.

You could just hardcode the value for simplicity.

:::

### Kernel 2: Moarrr threads!

With the first kernel, we're only able to compute small square matrices due to limits on
the number of workgroups you can dispatch at once.

Since we're launching one workgroup per entry, a 256x256 matrix is larger than our
limit!

Remember this line?

import { RustNaiveWorkgroup } from './snippets/naive.tsx';

<RustNaiveWorkgroup/>

We can reduce the number of dispatched workgroups by increasing the number of threads per workgroup!

If we update our GPU code

import { RustWorkgroup256Workgroup } from './snippets/workgroup_256.tsx';

<RustWorkgroup256Workgroup/>

we can reduce the number of total dispatched workgroups per dimension:

import { RustWorkgroup256WorkgroupCount } from './snippets/workgroup_256.tsx';

<RustWorkgroup256WorkgroupCount/>

With these two small changes we can handle larger matrices without hitting hardware
workgroup limits.

### Kernel 3: Calculating with 2D workgroups

However, doing all the computation in "1 dimension" still limits the matrix size we can
calculate.

Although we don't change much about our code, if we distribute our work in 2 dimensions
we're able to bypass these limits and launch more workgroups that are larger. This
allows us to calculate a 4096x4096 matmul.

We update our `compute(threads(256)))` to `compute(threads((16, 16)))`, and make the small
change to `row` and `col` from Zach's post to increase speed:

import { RustWorkgroup2d } from './snippets/workgroup_2d.tsx';

<RustWorkgroup2d />

And we need to tweak the workgroup dispatch count calculation on the CPU as we are in 2D
now and using the `y` value:

import { RustWorkgroup2dWorkgroupCount } from './snippets/workgroup_2d.tsx';

<RustWorkgroup2dWorkgroupCount />

### Kernel 4: Kernel tiling

Another thing to consider is how much work each thread does.

Up to now, each thread only computes one entry. But there is some overhead to launching
each workgroup versus computing more than 1 element per thread!

If calculating more elements per thread is faster than the overhead to launch each
workgroup, we should see a big speedup.

To do so, we calculate 4 results per thread (e.g. a 1x4 Tile).

import { RustTiling1d } from './snippets/tiling_1d.tsx';

<RustTiling1d />

The kernel looks roughly the same as before except we've unrolled the computation and
are calculating `TILE_SIZE` results per thread.

We can take this a step further and calculate 2D results per thread! Instead of
calculating 4 elements per single row, we can calculate 4 elements for 4 rows (e.g. a 2D
tile).

import { RustTiling2dSimd } from './snippets/tiling_2d_simd.tsx';

<RustTiling2dSimd />

Each thread now calculates a 4x4 grid of the output matrix and we see a slight
improvement over the last kernel.

To stay true to the spirit of Zach's original blog post, we'll wrap things up here and
leave the "fancier" experiments for another time.

## Reflections on porting to Rust GPU

Porting to Rust GPU went quickly, as the kernels Zach used were fairly simple. Most of
the time was spent with concerns that were not specifically about writing GPU code. For
example, deciding how much to abstract vs how much to make the code easy to follow, if
everything should be available at runtime or if each kernel should be a compilation
target, etc. [The
code](https://github.com/Rust-GPU/rust-gpu.github.io/tree/main/blog/2024-11-21-optimizing-matrix-mul/code)
is not _great_ as it is still blog post code!

My background is not in GPU programming, but I do have Rust experience. I joined the
Rust GPU project because I tried to use standard GPU languages and knew there must be a
better way. Writing these GPU kernels felt like writing any other Rust code (other than
debugging, more on that later) which is a huge win to me. Not just the language itself,
but the entire development experience.

## Rust-specific party tricks

Rust lets us write code for both the CPU and GPU in ways that are often impossible—or at
least less elegant—with other languages. I'm going to highlight some benefits I
experienced while working on this blog post.

### Shared code across GPU and CPU

In GPU programming, we often need to pass data between the CPU and GPU. For example, our
GPU kernel expects a `Dimensions` struct as input:

import { RustPartySettings } from './snippets/party.tsx';

<RustPartySettings />

We create an instance of `Dimensions` on the CPU and send it to the GPU via `wgpu`,
where the Rust kernel loads and uses it.

import { RustWgpuDimensions } from './snippets/party.tsx';

<RustWgpuDimensions />

This means the code on the CPU and GPU need to agree on the definition of
`Dimensions`!

In many GPU programming ecosystems, this would involve manually keeping the
definitions in sync across different languages—one for the CPU, one for the GPU. This is
tedious and error-prone.

With Rust, it's straightforward: we move the `Dimensions` struct into its own crate, and
both the CPU and GPU code depend on that crate. Now, the type definition lives in one
place and both platforms use it directly.

This approach eliminates duplication and guarantees consistency. If we need to make
changes, those changes propagate to both the CPU and GPU automatically, reducing the
risk of mismatches and making refactoring far safer.

This kind of consistency across CPU and GPU is something you don't often see in other
GPU programming ecosystems. Bespoke codegen solutions are often created to accomplish
the same thing Rust has built in.

### Running and debugging shaders on the CPU

GPU code can be notoriously hard to debug. While developing this kernel, I ran into a
bug I couldn't figure out. GPU debugging tools are limited and `printf`-style debugging
often isn't available. But what if we could run the GPU kernel _on the CPU_, where we
have access to tools like standard debuggers and good ol' `printf`/`println`?

With Rust GPU, this was straightforward. By using `cfg()` directives I made the
GPU-specific annotations (`#[spirv(...)]`) disappear when compiling for the CPU. The
result? The kernel became a regular Rust function. On the GPU, it behaves like a shader.
On the CPU, it's just a function you can call directly.

Here's what it looks like in practice using the 2D tiling kernel from before:

import { RustIsomorphic } from './snippets/party.tsx';

<RustIsomorphic />

The logic in the kernel hasn't changed, it is exactly the same as the GPU-only code from
before.

You'll also notice that on the GPU it uses `glam` from `spirv_std` but on the CPU it
uses `glam` from crates.io:

import { RustIsomorphicGlam } from './snippets/party.tsx';

<RustIsomorphicGlam />

This is enabled by the standard Rust ecosystem tooling
around dependencies:

import { RustIsomorphicDeps } from './snippets/party.tsx';

<RustIsomorphicDeps />

Testing the kernel in isolation is useful, but it does not reflect how the GPU executes
it with multiple invocations across workgroups and dispatches. To test the kernel
end-to-end, I needed a test harness that simulated this behavior on the CPU.

Building the harness was straightforward due to the borrow checker. By enforcing the
same invariants as the GPU I could validate the kernel under the same conditions the GPU
would run it:

import { RustCpuBackendHarness } from './snippets/party.tsx';

<RustCpuBackendHarness />

:::warning

Again, this code appears more complicated than it needs to be. I abstracted the CPU
testing harness code using generics and traits so I could easily slot in different
kernels and their settings while writing the blog post.

You could just call the kernel function directly in nested loops.

:::

### Tests

By moving the kernel code to the CPU, I could write tests that ran quickly and entirely
on the CPU. This eliminated the need to serialize tests and offload them to the GPU
(which is a shared and limited resource).

This approach has several benefits. First, it significantly reduced the feedback loop
during development, allowing me to catch issues faster. Second, it ensured the tests
could be run in any environment where the Rust toolchain is available—no GPU required.
This is especiallly relevant in CI environments such as Github Actions that do not have
a GPU by default.

For example, my test for a small matrix multiplication kernel running in the harness on
the CPU looked like this:

import { RustCpuBackendTest } from './snippets/party.tsx';

<RustCpuBackendTest />

### Benchmarks

I wanted to run benchmarks similar to those in the original blog post. Because I was
using Rust, this was simple. I used
[criterion](https://github.com/bheisler/criterion.rs) with `cargo bench`, just like any
other Rust project.

This required no new tools or workflows. The tools I already knew worked seamlessly.
More importantly, this approach benefits anyone working on the project. Any Rust
engineer can run these benchmarks with no additional setup--`cargo bench` is a standard
part of the Rust ecosystem.

### Lint

Linting GPU code in Rust works the same way as for CPU code. Running `cargo clippy`
highlighted issues and enforced consistent code quality. Any custom lint configurations
are also applied to Rust GPU kernels. Lints ensure that GPU code is held to the same
high standards as the rest of the project.

### Documentation

Writing doc comments and running `cargo doc` generates documentation for GPU kernels,
exactly how it happens in regular Rust. While some ecosystems offer similar tools,
Rust's integration is built-in and works seamlessly for both CPU and GPU code. There's
no special setup required.

## But wait, there's more!

The kernel in Zach's blog post is intentionally simple. That makes it easy to follow,
but it also means the Rust code looks very similar to WGSL. While this is fine for an
introductory example, it doesn't demonstrate Rust's real strengths for GPU programming.
These strengths—reusing existing libraries, traits, enums, generics, and more—become much more important as projects
grow in complexity.

### Leverage the existing Rust ecosystem

Rust's `no_std` ecosystem offers a wide array of libraries that can be used in
environments without the standard library. Traditionally this has meant embedded
devices, but a lot of the same assumptions apply to GPUs! As a consequence, you can
reuse [existing `no_std` & no `alloc` libraries from
crates.io](https://crates.io/categories/no-std::no-alloc) in your GPU code _without the
authors explicitly adding GPU support_. This is uniquely enabled by Rust GPU's
implementation choices and Rust's
[registers](https://without.boats/blog/the-registers-of-rust/). Sharing and reusing code
from the greater Rust ecosystem is a superpower when writing GPU programs that will
massively compound over time.

### Traits

Traits are one of Rust's most powerful tools and they work with Rust GPU. Traits let you
define zero-cost reusable type-safe behavior. For example, if you have multiple kernels
for different matrix multiplication strategies, you can define a `MatrixMultiplication`
trait and implement it for each variation. This eliminates duplication and makes your
code easier to extend.

### Enums and zero-sized types

GPU code is notoriously hard to read, but Rust's enums and zero-sized types (ZSTs) can
make it much more understandable. Enums let you explicitly encode states or modes. For
example, you can define tiling strategies or precision levels using enums instead of
relying on constants or magic numbers.

ZSTs take this further by encoding configurations directly into the type system. For
example, you could represent different kernel configurations as ZSTs. This approach
ensures invalid configurations are impossible, improving both readability and safety.

### Generics

Generics are another feature missing from this kernel but are a powerful tool in Rust
GPU. They allow you to write flexible kernels that work across different data types or
memory layouts. For instance, you can write a single function that supports both `f32`
and `f64` without duplicating code, all while maintaining type safety and performance.

### Error handling with `Result`

Rust GPU also supports error handling using `Result`. Encoding errors in the type system
makes it clear where things can go wrong and forces developers to handle those cases.
This is particularly useful for validating kernel inputs or handling the many edge cases
in GPU logic.

### Iterators

Rust's iterators don't appear in this kernel, but they're another way Rust GPU
simplifies complex logic. Instead of manual loops with indices, you can use iterators to
express your logic more clearly.

Iterators reduce the chance of off-by-one errors and make the intent of the code much
clearer.

Rust GPU's support for iterators is not complete but we are looking to improve it in the
future.

### Conditional compilation

This kernel doesn't use conditional compilation, but it's a key feature of Rust that
works with Rust GPU. With `#[cfg(...)]`, you can adapt kernels to different hardware or
configurations without duplicating code. GPU languages like WGSL or GLSL offer
preprocessor directives, but these tools lack standardization across projects. Rust GPU
leverages the existing Cargo ecosystem, so conditional compilation follows the same
standards all Rust developers already know.

## Come join us!

Rust GPU only recently became a [community managed
project](/blog/transition-announcement). We're eager to add more users and contributors!
We will be working on revamping the onboarding and documentation soon. To follow along
or get involved, check out the [`rust-gpu` repo on
GitHub](https://github.com/rust-gpu/rust-gpu).
<br/>

[^1]: Via [MoltenVK](https://github.com/KhronosGroup/MoltenVK)
[^2]:
    Technically `wgpu` translates SPIR-V to GLSL or WGSL via
    [naga](https://github.com/gfx-rs/wgpu/tree/trunk/naga)
