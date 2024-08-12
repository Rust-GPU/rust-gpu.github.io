+++
title = "Ecosystem"
weight = 1
+++

The GPU ecosystem in Rust is nascent and complex. It can be challenging to understand
why projects exist and how they compare to others in the space. This section is a
decoder ring for key projects that relate to both Rust and GPUs in some way.

# [rust-gpu](https://github.com/EmbarkStudios/rust-gpu)

Compiles unmodified Rust code to
[SPIR-V](https://www.khronos.org/spir/) (Vulkan) so that it can run on the GPU.

# [rust-cuda](https://github.com/Rust-GPU/Rust-CUDA)

Compiles unmodified Rust code to
[NVVM](https://docs.nvidia.com/cuda/nvvm-ir-spec/index.html) (CUDA) so that it can run
on the GPU. _Currently unmaintained but stay tuned!_

# [nvptx backend in rustc](https://doc.rust-lang.org/rustc/platform-support/nvptx64-nvidia-cuda.html)

Compiles unmodified Rust code to
[PTX](https://docs.nvidia.com/cuda/parallel-thread-execution/index.html) (CUDA) so that
it can run on the GPU.

# [CubeCL](https://github.com/tracel-ai/cubecl)

Compute language extension for Rust. Enables annotated Rust code to run on the GPU.

# [krnl](https://github.com/charles-r-earp/krnl)

Safe, portable, high performance compute (GPGPU) kernels. Enables annotated Rust code to
run on the GPU (via Rust GPU).

# [cudarc](https://github.com/coreylowman/cudarc)

Minimal and safe api over the CUDA toolkit. Enables calling CUDA from Rust running on
the CPU.

# [wgpu](https://wgpu.rs/)

A safe and portable graphics library for Rust based on the WebGPU API. Enables
communicating with the GPU from Rust running on the CPU.

# [naga](https://github.com/gfx-rs/wgpu/tree/trunk/naga)

A source code translator written in Rust that converts between various GPU-specific
shader languages ([WGSL](https://www.w3.org/TR/WGSL/),
[GLSL](https://en.wikipedia.org/wiki/OpenGL_Shading_Language),
[HLSL](https://en.wikipedia.org/wiki/High-Level_Shader_Language), and
[MSL](https://developer.apple.com/metal/)). These shaders ultimately run on the GPU.

# [ZLUDA](https://github.com/vosen/ZLUDA)

A drop-in replacement for CUDA written in Rust. Enables using tools and libraries
targeted at CUDA on non-NVIDIA hardware.

# [Asahi Linux GPU drivers](https://asahilinux.org/2022/12/gpu-drivers-now-in-asahi-linux/)
GPU drivers partially written in Rust to add Linux support for Apple GPUs.
