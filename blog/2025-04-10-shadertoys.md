---
title: "Shadertoys ported to Rust GPU"
authors: [LegNeato]
tags: ["demo"]
---

# Porting Shadertoys to Rust with Rust GPU

We ported a few popular [Shadertoy](https://www.shadertoy.com/) shaders over to Rust
using [Rust GPU](https://github.com/Rust-GPU/rust-gpu/). The process was straightforward
and we want to share some highlights.

<!-- truncate -->

The code is available on [GitHub](https://github.com/Rust-GPU/rust-gpu-shadertoys).

![Shadertoy screenshot](/img/shadertoys.jpg)

## What is Rust GPU?

[Rust GPU](https://rust-gpu.github.io/) is a project that allows you to write code for
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

For more details, check out the [Rust GPU website](http://rust-gpu.github.io/) or the
[GitHub repository](https://github.com/rust-gpu/rust-gpu).

## Shared code between CPU and GPU

Sharing data between the CPU and GPU is common in shader programming. This often
requires special tooling or manual effort. Using Rust on both sides made this seamless:

```rust
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub time: f32,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub drag_start_x: f32,
    pub drag_start_y: f32,
    pub drag_end_x: f32,
    pub drag_end_y: f32,
    pub mouse_left_pressed: u32,
    pub mouse_left_clicked: u32,
}
```

Note that on both the CPU and the GPU we are using the
[`bytemuck`](https://github.com/Lokathor/bytemuck) crate for the `Pod` and `Zeroable`
derives. This crate is unmodified and integrated directly from
[crates.io](https://crates.io/crates/bytemuck). Many `no_std` + no `alloc` Rust crates
work on the GPU!

## Traits, generics, and macros

Rust GPU supports traits. We used traits to encapsulate shader-specific operations in
reusable ergonomic abstractions:

```rust
pub trait FloatExt {
    fn gl_fract(self) -> Self;
    fn rem_euclid(self, rhs: Self) -> Self;
    fn gl_sign(self) -> Self;
    fn deg_to_radians(self) -> Self;
    fn step(self, x: Self) -> Self;
}
```

While there are still some rough edges, generics mostly work as expected. We used them
to support multiple channel types without duplicating logic:

```rust
pub struct State<C0, C1> {
    inputs: Inputs<C0, C1>,
    cam_point_at: Vec3,
    cam_origin: Vec3,
    time: f32,
    ldir: Vec3,
}
```

Rust macros also function normally. Using macros allowed us to reduce repetitive code
further.

```rust
macro_rules! deriv_impl {
    ($ty:ty) => {
        impl Derivative for $ty {
            deriv_fn!(ddx, OpDPdx, false);
            deriv_fn!(ddx_fine, OpDPdxFine, true);
            deriv_fn!(ddx_coarse, OpDPdxCoarse, true);
            deriv_fn!(ddy, OpDPdy, false);
            deriv_fn!(ddy_fine, OpDPdyFine, true);
            deriv_fn!(ddy_coarse, OpDPdyCoarse, true);
            deriv_fn!(fwidth, OpFwidth, false);
            deriv_fn!(fwidth_fine, OpFwidthFine, true);
            deriv_fn!(fwidth_coarse, OpFwidthCoarse, true);
        }
    };
}

// Applied easily to multiple types:
deriv_impl!(f32);
deriv_impl!(Vec2);
deriv_impl!(Vec3A);
deriv_impl!(Vec4);
```

## Standard Rust tools

Want to typecheck the shaders? `cargo check`. Build them? `cargo build`. Run in release
mode? `cargo run --release`. Gate code at compile time? Use
[features](https://doc.rust-lang.org/cargo/reference/features.html).

If you run `clippy` on the shaders, you'll see it complains about many things as we
intentionally kept the Rust versions of shaders similar to their original GLSL versions.

This is one of Rust GPU's big advantages: you can use all the Rust tools you're already
familiar with.

## Improving the Rust ecosystem

While porting shaders, we also contributed back to the ecosystem by identifying and
fixing several issues in [`wgpu` and `naga`](https://github.com/gfx-rs/wgpu):

- [Fixed a panic while processing SPIR-V](https://github.com/gfx-rs/wgpu/pull/7397)
- [Fixed incorrect translation of certain shader literals when targeting
  Metal](https://github.com/gfx-rs/wgpu/pull/7437)
- [Fixed a regression making it impossible to include raw
  SPIR-V](https://github.com/gfx-rs/wgpu/pull/7503)

These fixes help everyone using `wgpu` and `naga`, not just users of Rust GPU.

## Come join us!

While we hit some sharp edges, porting Shadertoy shaders to Rust with Rust GPU was
reasonably straightforward. Rust GPU is definitely ready for shader experimentation.

We're eager to add more users and contributors! We will be working on revamping the
onboarding and documentation soon. To follow along or get involved, check out the
[`rust-gpu` repo on GitHub](https://github.com/rust-gpu/rust-gpu).
