---
title: "Rebooting the Rust CUDA project"
authors: [LegNeato]
tags: ["announcement", "cuda"]
---

We're excited to announce the reboot of the [Rust CUDA project](https://github.com/rust-gpu/rust-cuda). Rust CUDA enables you to write and run [CUDA](https://developer.nvidia.com/cuda-toolkit) kernels in Rust, executing directly on NVIDIA GPUs using [NVVM IR](https://docs.nvidia.com/cuda/nvvm-ir-spec/index.html).

Our immediate focus is on modernizing the project and integrating with other GPU-related efforts in the Rust ecosystem.

**To follow along or get involved, check out the [`rust-cuda` repo on GitHub](https://github.com/rust-gpu/rust-cuda).**

<!-- truncate -->

## Call for contributors

We need your help to shape the future of CUDA programming in Rust. Whether you're a
maintainer, contributor, or user, there's an opportunity to [get
involved](https://github.com/rust-gpu/rust-cuda). We're especially
interested in adding maintainers to make the project sustainable.

Be aware that the process may be a bit bumpy as we work to get the project in order.

## Relation to other GPU projects in the Rust ecosystem

We see significant opportunities for collaboration and unification with related
projects.

- **[Rust GPU](https://rust-gpu.github.io/)**: This project is similar to Rust CUDA but
  targets [SPIR-V](https://www.khronos.org/spir/) for [Vulkan](https://www.vulkan.org/)
  GPUs.

  Our long-term vision for the two projects includes merging developer-facing APIs to
  provide a unified experience for GPU programming in Rust. Developers should not need
  to worry about underlying platforms (Vulkan vs. CUDA) or low-level IR (SPIR-V vs.
  NVVM) unless doing specialized work. This would make GPU programming in Rust feel as
  seamless as its abstractions for CPU architectures or operating systems.

  Christian Legnitto ([LegNeato](https://github.com/LegNeato)) is one of the maintainers
  for both projects.

- **[`rustc` PTX
  backend](https://doc.rust-lang.org/rustc/platform-support/nvptx64-nvidia-cuda.html)**:
  The Rust compiler's experimental `nvptx` backend generates CUDA's low-level
  [PTX](https://docs.nvidia.com/cuda/parallel-thread-execution/) IR. We plan to
  collaborate with this effort to determine how it can complement or integrate with
  `rust-cuda`'s NVVM IR approach.

- **[cudarc](https://github.com/coreylowman/cudarc)**: This crate provides a host-side
  abstraction for CUDA programming in Rust. We aim to explore how `rust-cuda` can
  interoperate with `cudarc` or consolidate efforts.

For a broader look at Rust and GPUs, check out the [ecosystem overview
page](https://rust-gpu.github.io/ecosystem/).

## Short-Term Roadmap

Our immediate focus is on stabilizing and modernizing the Rust CUDA project:

- **Merging outstanding PRs**: Address the backlog of contributions.
- **Closing outdated issues**: Clean up the issue tracker to focus on actionable problems.
- **Updating dependencies**: Support the latest Rust toolchain and CUDA versions.
- **New website**: Launch a site with updated documentation, examples, and resources to make it easier for users to get started.

## Medium-term roadmap

Once the Rust CUDA project is stabilized, we will explore new opportunities to advance
its functionality and integrate with the broader ecosystem:

- **Rust-C++ interop**: Investigate Rust's recent C++ interoperability initiatives to
  see if they can be used to simplify host and library integration for CUDA.
- **Rust community collaboration**: Work with other Rust CUDA projects to consolidate
  and unify host-side tools and workflows where appropriate.
- **PTX backend collaboration**: Coordinate with the [rustc PTX
  backend](https://doc.rust-lang.org/rustc/platform-support/nvptx64-nvidia-cuda.html)
  team to explore shared infrastructure and evaluate transitioning Rust CUDA from NVVM
  IR to raw PTX.
- **`rust-gpu` collaboration**: Leverage `rust-gpu`'s tooling and compiler
  infrastructure to reduce duplication.

## Long-Term Roadmap

Our long-term vision is to make GPU programming in Rust safe, ergonomic, fast, and fully
integrated into the language:

- **Rust compiler integration**: Rust CUDA is currently an out-of-tree compiler backend.
  We aim to bring it in-tree and make it officially supported.
- **Unified GPU API**: Merge developer-facing APIs of `rust-cuda` and `rust-gpu`. This
  would abstract away platform and IR differences for most developers, leaving
  platform-specific concerns only for low-level or specialized use cases.
- **Language evolution**: Contribute to the Rust language's evolution to better support
  GPU programming, ensuring it remains safe, performant, and developer-friendly.

## Acknowledgments

We want to extend our heartfelt thanks to [Riccardo
D'Ambrosio](https://github.com/RDambrosio016) for starting the Rust CUDA project and
putting in so much work to bring it to life. His efforts have laid the foundation for
everything we are building on today. **Thank you, Riccardo!**
