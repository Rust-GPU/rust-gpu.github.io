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



## Traditional LOD

TODO pic of some low poly 3D model

// A 3D model is represented by a List of indices and a List of vertices. 
3D models for real-time rendering are made up of many triangles describing its surface and many vertices, which are the corners of our triangles. Vertices are highly configurable with the properties they have, but at the very least need to have a position of where they are. Triangles on the other hand are simply a list of indices pointing into the List of vertices, where a set of 3 form a single triangle.

When rasterizing such a model to the screen, we use a vertex shader to transform our vertices . 

The cost of rendering a 3D model is typically the amount of pixels on screen plus the amount of vertices of the model. 

## Terrain Generation

## Nanite

## rust-gpu-bindless

## Epilog
