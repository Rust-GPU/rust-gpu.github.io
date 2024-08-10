+++
title = "Rust GPU"
sort_by = "weight"
+++

Rust GPU makes it possible to write and run GPU software in Rust, leveraging the
language's powerful safety and concurrency features to enhance performance and
reliability. With Rust GPU, you can seamlessly develop for both CPU and GPU using a
unified codebase, all while benefiting from Rust’s existing ecosystem.

## Multi-Vendor

The Rust GPU compiler backend emits code compatible with
[Vulkan](https://www.vulkan.org), ensuring your code runs across a wide range of devices
and vendors.

If instead you wish to stick to the NVIDIA ecosystem, stay tuned as the [Rust
CUDA](https://github.com/rust-gpu/rust-cuda) project is in the process of being rebooted
and possibly integrated with Rust GPU.

## Modern and Unified

There is no longer a need to learn a GPU-specific programming language. You can write
both CPU and GPU code in Rust, leveraging your existing Rust knowledge and maintaining a
consistent development experience. Furthermore, the _same code can run on both the CPU
and GPU_, with divergent behavior gated behind `cfg` attributes and macros where
necessary.

Even if your current codebase isn't written in Rust, choosing Rust for the GPU parts
gives you more widely applicable skills in one of the [fastest-growing languages on
GitHub](https://octoverse.github.com/2022/top-programming-languages). Rust is also one
of the [most admired programming
languages](https://github.blog/developer-skills/programming-languages-and-frameworks/why-rust-is-the-most-admired-language-among-developers/)
while GPU-specific languages are considered a necessary evil.

## Fearless Concurrency

Rust's ownership model and type system guarantee memory
safety, minimizing bugs and undefined behavior. Rust's borrow checker also enables
fearless concurrency, which is essential for maximizing performance on massively
parallel GPUs.

## Powerful Abstractions

Programming in GPU-specific languages can often feel like [taking a step back to the
90s](https://xol.io/blah/death-to-shading-languages/), where primitive tools and sharp
edges abound. Because of this, code written for GPUs is simplistic with low cyclomatic
complexity. Rust has an expressive type system and zero-cost abstractions that enable
writing high-level, reusable code without sacrificing performance. This approach leads
to more maintainable and sophisticated GPU programs, streamlining the development
process and enhancing productivity.

## Existing Ecosystem

The state-of-the-art for sharing GPU code is copy
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
