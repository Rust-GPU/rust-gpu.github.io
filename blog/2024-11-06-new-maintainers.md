---
title: "Welcoming two new rust-gpu maintainers"
authors: [schell, firestar99]
tags: ["announcement", "leadership"]
---

# Welcoming two new rust-gpu maintainers

We’re thrilled to introduce two new maintainers for the
[rust-gpu](https://github.com/Rust-GPU/rust-gpu) project: **[Schell
Scivally](https://github.com/schell/)** and **[Firestar99](https://github.com/Firestar99)**.

Both bring experience in graphics programming, a deep understanding of Rust, and a
passion for advancing GPU computing.

<!-- truncate -->

Here’s a bit more about each of them, in their own words:

## Meet Schell Scivally

> I'm Schell Scivally, a game and graphics programmer living on the South Island of New
> Zealand 🇳🇿🌴 with my partner and our two kids.
>
> By day, I work at [Narrative.so](https://narrative.so), where I use AI to streamline
> photographers' culling and editing. Before moving here, I was a Haskell programmer in
> the Bay Area of California.
>
> I’m also an [NLNet grantee](https://nlnet.nl/) working on Renderling, a cross-platform
> GPU-driven renderer that runs on WebGPU. You can check it out at
> [renderling.xyz](https://renderling.xyz).
>
> I use rust-gpu to debug shaders on the CPU while keeping all the benefits of Rust’s
> development experience. It lets me shift a lot of work from the CPU to the GPU without
> having to change or duplicate types! I’d love to see rust-gpu grow, get more
> contributors, and become the foundation for a thriving shader ecosystem on crates.io.

## Meet Firestar99

> I go as Firestar99 online and am a graphics programmer and master's student from
> Germany 🇩🇪.
>
> Currently, I'm in the process of finishing off my master’s degree with a thesis about
> replicating UE5's Nanite in a custom engine written in rust. For those unfamiliar,
> Nanite is a new approach to generate and render models with levels of details (LOD),
> being the first to allow a single instance of a mesh to transition smoothly from
> higher to lower levels of detail as it gets further away from the camera, without any
> visible holes or seams in the model. In the context of rust, you may have heard of
> [bevy implementing it as
> well](https://jms55.github.io/posts/2024-06-09-virtual-geometry-bevy-0-14/).
>
> And of course I'm using rust-gpu for my shaders! By far the most important feature to
> me is the ability to share datastructures across GPU and CPU code, which allows me to
> preprocess my models in a seconds long offline bake directly into the GPU
> representation, enabling me to load even the most complex models like
> [bistro](https://developer.nvidia.com/orca/amazon-lumberyard-bistro) in half a second.
>
> On the way there I've made various additions to rust-gpu like [Mesh
> shaders](https://github.com/Rust-GPU/rust-gpu/pull/44), [subgroup
> intrinsics](https://github.com/Rust-GPU/rust-gpu/pull/14) and [reworking
> `ByteAddressableBuffer`](https://github.com/Rust-GPU/rust-gpu/pull/17) as well as
> rebasing a lot of existing PRs. In the future, I hope to be able to contribute my
> bindless API specifically designed for rust-gpu and find a job in the industry,
> hopefully associated with the project.

<br/>

Please join us in welcoming them to the team! Thanks to our new maintainers, `rust-gpu`
now has four official maintainers.

We’re eager to add more users and contributors to `rust-gpu` and will be working on
revamping the onboarding and documentation soon. **To follow along or get involved,
check out `rust-gpu` on [GitHub](https://github.com/rust-gpu/rust-gpu).**
