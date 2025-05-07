---
title: "Nanite at Home using Rust-GPU"
authors: ["firestar99"]
slug: nanite-at-home
tags: ["demo", "code", "performance"]
---

Why pay Unreal Engine money for their Nanite, when we have Nanite at Home?

How hard could it be to replicate it ourselves, with rust-gpu?

TODO Nanite at home meme?



<!-- truncate -->



## Traditional Level of Detail

TODO pic of some low poly 3D model

3D meshes for real-time rendering are made up of many triangles and their corners, called vertices.
At a minimum a vertex must define its position but can have various other properties, with their memory layout being fully dynamic. 
Triangles on the other hand are simply a list of u16 or u32 indices pointing into the List of vertices, where a set of 3 form a single triangle.

TODO quad example?

The process of rasterising our model into colorful pixels on screen consists out of a 3 step process:
* The vertex shader calculates the position of our vertex from the camera's perspective and do other kinds of per-vertex transformations. 
* Then our triangle is rasterized into the individual pixel locations.
* The fragment shader evaluates the color of every pixel.

The cost of rendering scales largely by the shaders that need to be run, or in other words: The amount of pixels on screen plus the amount of vertices of the model. 

As we move a model further away from the camera, the mesh gets smaller and fewer fragment shader need to be evaluated. However, we would still need to call the vertex shader for every single vertex to know where it ends up on screen, even if the detail they describe would be too small to notice. To improve performance, it is common practice to not just have a single mesh, but to create multiple meshes at different detail levels that can be swapped out, usually depending on the distance to the camera.

The reducing vertices in a model can in part be automated and is called "mesh simplification". Level of Detail (LOD)

## Terrain Generation

## Nanite

## rust-gpu-bindless

## Epilog
