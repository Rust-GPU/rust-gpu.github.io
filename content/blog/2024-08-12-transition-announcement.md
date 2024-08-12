+++
title = "Rust GPU Transitions to Community Ownership"
authors = ["LegNeato", "eddyb"]
+++

We are excited to announce that as of today the <a
href="https://rust-gpu.github.io">Rust GPU</a> project will be transitioning from
[Embark Studios](https://www.embark-studios.com/) to community ownership under the [Rust
GPU GitHub organization](https://github.com/rust-gpu/rust-gpu). This move marks the
beginning of a broader strategy aimed at revitalizing, unifying, and standardizing GPU
programming in Rust. We are eager to share more details in the near future but in the
meantime if you are a maintainer of a GPU-related Rust project, please reach out!

## What is Rust GPU?

<a href="https://rust-gpu.github.io">Rust GPU</a> makes it possible to write and run GPU
software in Rust, leveraging the language's powerful safety and concurrency features to
enhance performance and reliability. With Rust GPU, you can seamlessly develop for both
CPU and GPU using a unified codebase, all while benefiting from Rust’s existing
ecosystem.

The Rust GPU compiler backend emits code compatible with
[Vulkan](https://www.vulkan.org), ensuring your code runs across a wide range of devices
and vendors.

If instead you wish to stick to the NVIDIA ecosystem, stay tuned as the [Rust
CUDA](https://github.com/rust-gpu/rust-cuda) project is in the process of being rebooted
and possibly integrated with Rust GPU.

To put Rust GPU in context, here's an overview of the current landscape:

- [rust-gpu](https://github.com/EmbarkStudios/rust-gpu): Compiles unmodified Rust code to
  [SPIR-V](https://www.khronos.org/spir/) (Vulkan) so that it can run on the GPU.
- [rust-cuda](https://github.com/Rust-GPU/Rust-CUDA): Compiles unmodified Rust code to
  [NVVM](https://docs.nvidia.com/cuda/nvvm-ir-spec/index.html) (CUDA) so that it can run
  on the GPU. _Currently unmaintained but stay tuned!_
- [nvptx backend in
  rustc](https://doc.rust-lang.org/rustc/platform-support/nvptx64-nvidia-cuda.html):
  Compiles unmodified Rust code to
  [PTX](https://docs.nvidia.com/cuda/parallel-thread-execution/index.html) (CUDA) so
  that it can run on the GPU.
- [CubeCL](https://github.com/tracel-ai/cubecl): Compute language extension for Rust.
  Enables annotated Rust code to run on the GPU.
- [krnl](https://github.com/charles-r-earp/krnl): Safe, portable, high performance
  compute (GPGPU) kernels. Enables annotated Rust code to run on the GPU (via Rust GPU).
- [cudarc](https://github.com/coreylowman/cudarc): Minimal and safe api over the CUDA
  toolkit. Enables calling CUDA from Rust running on the CPU.
- [wgpu](https://wgpu.rs/): A safe and portable graphics library for Rust based on the
  WebGPU API. Enables communicating with the GPU from Rust running on the CPU.
- [naga](https://github.com/gfx-rs/wgpu/tree/trunk/naga): A source code translator
  written in Rust that converts between various GPU-specific shader languages
  ([WGSL](https://www.w3.org/TR/WGSL/),
  [GLSL](https://en.wikipedia.org/wiki/OpenGL_Shading_Language),
  [HLSL](https://en.wikipedia.org/wiki/High-Level_Shader_Language), and
  [MSL](https://developer.apple.com/metal/)). These shaders ultimately run on the GPU.
- [ZLUDA](https://github.com/vosen/ZLUDA): A drop-in replacement for CUDA written in
  Rust. Enables using tools and libraries targeted at CUDA on non-NVIDIA hardware.
- [Asahi Linux GPU
  drivers](https://asahilinux.org/2022/12/gpu-drivers-now-in-asahi-linux/): GPU drivers
  partially written in Rust to add Linux support for Apple GPUs.

## Why Rust for GPU Programming?

1. **Modern and Unified Experience:** There is no longer a need to learn a GPU-specific
   programming language. You can write both CPU and GPU code in Rust, leveraging your
   existing Rust knowledge and maintaining a consistent development experience.
   Furthermore, the _same code can run on both the CPU and GPU_, with divergent behavior
   gated behind `cfg` attributes and macros where necessary.

   Even if your current codebase isn't written in Rust, choosing Rust for the GPU parts
   instead of GPU-specific languages gives you more widely applicable skills in one of
   the [fastest-growing languages on
   GitHub](https://octoverse.github.com/2022/top-programming-languages). Rust is also
   one of the [most admired programming
   languages](https://github.blog/developer-skills/programming-languages-and-frameworks/why-rust-is-the-most-admired-language-among-developers/)
   while GPU-specific languages are considered a necessary evil.

2. **Fearless Concurrency:** Rust's ownership model and type system guarantee memory
   safety, minimizing bugs and undefined behavior. Rust's borrow checker also enables
   fearless concurrency, which is essential for maximizing performance on massively
   parallel GPUs.

3. **Powerful Abstractions:** Programming in GPU-specific languages can often feel like
   [taking a step back to the 90s](https://xol.io/blah/death-to-shading-languages/),
   where primitive tools and sharp edges abound. Because of this, code written for GPUs
   is simplistic with low cyclomatic complexity. Rust has an expressive type system and
   zero-cost abstractions that enable writing high-level, reusable code without
   sacrificing performance. This approach leads to more maintainable and sophisticated
   GPU programs, streamlining the development process and enhancing productivity.

4. **Leverage Existing Ecosystem:** The state-of-the-art for sharing GPU code is copy
   and pasting. With Rust GPU we are excited to bring the excellent `cargo` and
   crates.io ecosystem to GPU programming and provide some sanity.

   Additionally, Rust's `no_std` ecosystem offers a wide array of libraries that can be
   used in environments without the standard library. Traditionally this has meant
   embedded devices, but a lot of the same assumptions apply to GPUs! As a
   consequence, you can reuse [existing `no_std` libraries from
   crates.io](https://crates.io/categories/no-std::no-alloc) in your GPU code _without
   the authors explicitly adding GPU support_. This is uniquely enabled by Rust GPU's
   implementation choices and Rust's
   [registers](https://without.boats/blog/the-registers-of-rust/). Sharing and reusing
   code from the greater Rust ecosystem is a superpower when writing GPU programs that
   will massively compound over time.

   Soon developers will write and share Rust libraries that _require_ running on the
   GPU, similar to how some existing open source libraries require a specific OS or CPU
   architecture. We expect many innovative GPU-specific algorithms and projects to be
   built in Rust.

## A Heartfelt Thank You to Embark

First and foremost, we extend our deepest gratitude to [Embark
Studios](https://www.embark-studios.com/) for their incredible work in kickstarting and
supporting the Rust GPU project. Their dedication and contributions have been
invaluable, laying a strong foundation for the future. The community is immensely
grateful for all the resources and expertise Embark has poured into this project.

## Transition Challenges: Repository Transfer

Unfortunately, Embark has decided not to transfer the existing repository via [GitHub's
transfer
tool](https://docs.github.com/en/repositories/creating-and-managing-repositories/transferring-a-repository),
which means we will lose all our stars, issues, and pull requests. Additionally,
automatic GitHub redirects will not be added. All this causes avoidable churn.

We have not fully transitioned to the new community-owned repository due to these
downsides and we believe the best course of action is to encourage Embark to reconsider
and simply press the GitHub transfer button, easing this transition for everyone
involved. We appreciate the community's patience and understanding as we get things
sorted out and we'd love your support encouraging Embark to reconsider the mechanics of
the transfer.

## New Focus: GPU Compute and GPGPU

With the industry's growing emphasis on generative AI and LLMs, we are shifting our
primary focus towards GPU compute and General-Purpose GPU (GPGPU) programming. This also
aligns with the trend in GPU hardware to increasingly utilize the compute capabilities
of GPUs to run graphics pipelines as software libraries rather than relying on
specialized hardware. Even graphics-focused programs such as video games are moving away
from solely relying on graphics pipelines and instead running as much of the engine as
possible in GPU compute.

In the past, the primary focus on graphics use-cases did not materialize the necessary
contributors and community engagement to make the Rust GPU project sustainable. We hope
our new focus will attract more contributors and users.

However, it is important to note that graphics use-cases will continue to be supported,
developed, and improved in Rust GPU! We appreciate our early graphics adopters and
remain committed to enhancing the graphics capabilities of Rust GPU.

## Addressing Stability and Nightly Rust

Though Rust GPU requires the nightly version of Rust, Embark had unique stability goals
which resulted in Rust GPU not tracking the _latest_ nightly. This has been a hurdle for
adoption, as most people experimenting with Rust GPU are likely already on nightly to
get the latest and greatest, not use a randomly chosen nightly from months ago. To
address this, we will be tracking the latest Rust nightly builds moving forward. For
those concerned about stability, you can pin to a particular nightly version via <a
href="https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file">`rust-toolchain`</a>,
and there are ways to build your GPU code with nightly while keeping your CPU code on
stable.

With today's release we are closer to this goal but we aren't quite tracking the latest
nightly yet as some newer `rustc` changes require more substantial changes to Rust GPU's
compiler backend.

## Short-Term Roadmap

Our immediate focus is on the following tasks:

- **Merge all outstanding work.**
- **Track the latest Rust nightly.** Stay up-to-date with the latest developments in
  Rust and catch changes that break Rust GPU's compiler backend as soon as they land
  upstream.
- **Release more often.** Increase our release cadence to deliver improvements and fixes
  more frequently, as well as aid in debugging issues.
- **Extend/update documentation, FAQ, and guides.** Provide solutions to common problems
  and usage scenarios and document unsupported language constructs.
- **Better compatibility.**
  - Fully switch to [quasi-pointers](https://github.com/EmbarkStudios/spirt/pull/24) to
    support more advanced Rust constructs.
  - Add `alloc` support. This will allow us to move from supporting [at most 500
    `no_std`, no alloc crates](https://crates.io/categories/no-std::no-alloc) to [over
    6,000 `no_std` + alloc crates](https://crates.io/categories/no-std).
- **Better testing.**
  - Develop our own crater-like tool to create a burndown list of `no_std` and `alloc`
    crates that don't work with Rust GPU.
  - Implement a differential testing harness.
  - Establish ecosystem CI similar to the [SWC
    project](https://github.com/swc-project/swc/tree/main/.github/swc-ecosystem-ci).

## Long-Term Vision: First-Class GPU Targets in Rust

Looking further ahead, one of our ambitious goals is to merge Rust GPU or its successor
into the Rust compiler itself. We want to make GPUs (and TPUs and NPUs) first-class
targets in Rust and make programming them as ergonomic as CPU programming. **Hardware is
shifting from scalar, serial processing to matrix, parallel processing—software must do
the same.** We are excited about the future of Rust GPU and the potential it holds.

Thank you for your continued support and contributions. Stay tuned for more updates, and
don't hesitate to reach out if you have any questions or want to get involved.

Happy coding!

_LegNeato ([GitHub](https://github.com/LegNeato),
[Twitter](https://twitter.com/legneato)) and eddyb ([GitHub](https://github.com/eddyb))_

**Rust GPU Maintainers**
