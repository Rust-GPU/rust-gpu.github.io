---
title: "Nanite at Home"
authors: ["firestar99"]
slug: nanite-at-home
tags: ["demo", "code", "performance"]
---

TODO Nanite at home meme?

Why pay Unreal Engine money for their Nanite, when we have Nanite at Home?

How hard could it be to replicate it ourselves, with rust-gpu?



<!-- truncate -->

TODO proper introduction, why Terrain gen before nanite?    

## Triangles, Vertices, Meshes and Level of Detail

:::tip
Feel free to skip this chapter if you already know all these concepts, but I suspect there will be plenty of rust programmers unfamiliar with computer graphics.
:::

TODO pic of some low poly 3D model

You've probably all seen a 3D mesh made up of many triangles, but how do we actually represent them? The triangles themselves are quite simple, it's just a List of u16 or u32 indices where a set of 3 describe the corner IDs to connect to create a triangle. Far more interesting are these corners, we call them vertices, as they often cary not just the position they are located at, but you can attach various other attributes to them. For example, normals can describe the "up" direction of a surface, which is important in lighting calculations. Or texture coordinates to describe how 2D images should be wrapped around our 3D mesh, think of the wrapper around chocolate Easter bunnies. 

As we're writing a renderer, we don't just want to store models, we want to render them to the screen and look at it from different directions! 
For realtime applications you'd typically use a process called "rasterization" to turn a model into colorful pixels on screen, which includes programmable shaders allowing us to manipulate the appearance of our model:
1. The vertex shader runs once per vertex and calculates where our vertex would end up on the screen, but can also change the other attributes attached to the vertex. 
2. A hardware rasterizer assembles the triangles and figures out where they end up on the screen. It then creates a stream of fragments for each pixel the triangle overlaps.
3. The fragment shader evaluates the color of every emitted fragment.

The cost of rendering scales largely by the shaders that need to be run, or in other words: The amount of pixels on screen plus the amount of vertices of the model. 

TODO pic of some different LOD levels

As we move a model further away from the camera, the mesh gets smaller and fewer fragments need to be evaluated. However, we would still need to call the vertex shader for every single vertex to know where it ends up on screen, even if the detail they describe would be too small to notice. To improve performance, it is common practice to not just have a single mesh, but to create multiple meshes at different Level of Detail (LOD) that can be swapped out, depending on the distance to the camera. The process of reducing the amount of geometry of a mesh is called "mesh simplification", 



## Terrain in video games

![minecraft chunks](./minecraft_chunks.jpg)

You've all played or at least seen Minecraft with its infinite worlds made of blocks. We can't draw infinite amounts of geometry, so we need to segment the world into chunks and only load a small amount of them around the player. And as the player moves in some direction, we load new chunks there and unload the ones behind them.
But that alone doesn't lend itself to far view distances, as chunks further from the camera have just as much geometry as the one the player is standing in.
We need simplified chunks.

<figure>
![terrain_mesh_lod.jpg](terrain_mesh_lod.jpg)
<figcaption>[image source](https://www.researchgate.net/publication/342611763_General-Purpose_Real-Time_VR_Landscape_Visualization_with_Free_Software_and_Open_Data)</figcaption>
</figure>

Going away from blocks, the image above shows the typical geometry of a single chunk. The white lines indicate the geometry for a 64x64 grid, with a square being represented by 2 triangles each. Compare that to the red geometry representing the same chunk, but by using only an 8x8 grid it has far fewer geometric detail. So by using a smaller grid, we can generate simpler geometry.

<figure>
![cdlod_selection.jpg](./cdlod_selection.jpg)
<figcaption>Source: Continuous Distance-Dependent Level of Detail for Rendering Heightmaps</figcaption>
</figure>

A typical approach is to combine a 2x2 of chunks into one larger chunk at only a quarter of the vertex density. With 4 times as much physical area but a quarter of the density, the new simplified chunk have the exact same amount of triangles and vertices as our detailed chunks, making both chunks cost about the same to render. We can then repeat this process a bunch of times to create less and less detailed chunks, which will eventually lead us to a chunk system like shown above.
There are many physically small and very detailed chunks near the camera and larger, less detailed chunks the further away we get. With the different colors and sizes indicating the various levels of detail (LOD) of each chunk. 

![](./tikz/terrain_tree.png)

As we iteratively create more and more simplified chunks, you may notice that they're starting to form a tree. Specifically a quadtree, a binary tree in 2 dimensions, where one node splits into four new nodes. In the graph above, we're only visualizing it with two children per node to keep things clearer. To select which chunks to draw at which LOD, we can simply traverse the tree from top to bottom until we hit a node that is "detailed enough" from the view of the camera. Selecting these nodes creates a "cut" though the tree, as defined by graph theory. While evaluating this cut is quite simple in a tree, later on we will come back to calculating the cut in a different kind of graph, where it isn't as trivial.



## Terrain holes

import terrain_hole from './tikz/terrain_hole.png';

<figure style={{float:"left"}}>
<img src={terrain_hole} width="350"/>
</figure>

The biggest issue with LOD Terrain is the creation of holes between different detail levels. In the image above, imagine you are up high looking down on some terrain, with the orange chunk being closer to you and at a higher detail and the blue chunk being further away with less detail. On these LOD transitions, the geometry between the chunks usually doesn't perfectly align, which can result in some visual artifacts. In the lucky case on the left, we may notice that a hill is being cut off. But on the right side, the detailed vertices go below the simplified chunk's height, creating a hole into the void underneath! There's different approaches on how to deal with these holes, and we want to outline some of them here.

<figure>
![terrain_mesh_lod.jpg](terrain_mesh_lod.jpg)
<figcaption>[image source](https://www.researchgate.net/publication/342611763_General-Purpose_Real-Time_VR_Landscape_Visualization_with_Free_Software_and_Open_Data)</figcaption>
</figure>

TODO different image?

Skirts are a very easy way to circumvent, but not prevent holes. Around each of your chunks, you some extra geometry extending into the ground, creating a skirt. Usually you'd never actually notice the skirt being there, but if you were to look at a hole like in the scenario above, instead of staring into the void you'd be looking at the skirt. This hides the hole in the terrain, but you could still notice the hole by the texture or lighting mismatching with the surrounding terrain. Skirts are also a bad choice if you want to have caves beneath your terrain, as they could easily poke though the ceiling. 

The State of the Art in Terrain Rendering is Vertex Morphing, though we won't go into detail in this article.

<figure>
![](./tree_locked_border.jpg)
<figcaption>[image source](https://blog.traverseresearch.nl/creating-a-directed-acyclic-graph-from-a-mesh-1329e57286e5)</figcaption>
</figure>

We'd much rather focus on an approach that at first seems rather counterproductive: What if we never modify the vertices at the chunk borders? This would ensure that the borders between two chunks always match perfectly and have no possibility of ever forming a hole. However, as we simplify our chunks more and more, keeping the very detailed borders around would lead to many very small triangles, like in the image above. And many small triangles, especially elongated ones, are particularly bad for rasterization performance. But it's ability to guarantee no holes while allowing chunks to independently select their level of detail is a very useful property, which we'll exploit in Nanite. 


## Nanite

As we move from Terrain rendering to Nanite operating on general meshes, we have to replace our Chunks with a different concept: Clusters or Meshlets.
While nowadays both are used interchangeably, the original nanite presentation only used the term clusters. Clusters have long been used to describe a small subset of a mesh, usually limited by the amount of triangles it contains.
Whereas the term meshlet comes from the introduction of the new mesh shader. Primarily it's a compute shader that additionally can also emit arbitrary geometry to the rasterization hardware, replacing the vertex shader and any other optional stages before the rasterizer. But mesh shaders have a hardware-dependent limit on how many vertices and triangles can be emitted by a single workgroup. Thus, the term meshlet was born to describe any kind of subset of a mesh that fit within these limits. Vulkan guarantees that mesh shader support at least 64 vertices and 128 triangles, though in practise both AMD and Nvidia have converged on 128 vertices and 256 triangles as being their limits and the optimal size of a meshlet.

In the following visualizations, we will assume that our clusters have a maximum amount of 4 triangles and 6 vertices.

The very first step is to load our mesh from disk using the [gltf crate](https://github.com/gltf-rs/gltf) and turn it into clusters. Luckily, the popular mesh processing library [meshoptimizer](https://github.com/zeux/meshoptimizer) (with rust bindings from [meshopt-rs](https://github.com/gwihlidal/meshopt-rs)) has a convenient function called [`build_meshlets`](https://github.com/gwihlidal/meshopt-rs/blob/c2165927e09c557e717f6fcb6b7690bee65f6c90/src/clusterize.rs#L56) that will turn a mesh into clusters from  for you, in a pretty optimized way. Now that we have clusters, we can start the simplification loop:

![](./tikz/nanite_mesh_all.png)

1. Select a group of 4 clusters: This would get us the state on the left, with 4 clusters in different colors, and their respective cluster graph at the bottom. Think of it as selecting our "2x2 of chunks" which we want to simplify.
2. Merge the clusters into a single mesh: We forget about clusters for a moment, and just merge the clusters of our group into a single small mesh, which we'll need for the next step.
3. Simplify the clusters with fixed borders: This gets us to the middle state, where we have a white mesh that has been simplified. But most importantly, the outer borders to the surrounding groups are fixed and have not changed at all. This allows us to decide to draw the higher or lower LOD *independently* of our neighbours, which is critical to ensure we don't get any holes in our model though LOD transitions. In our cluster graph, we deonte it as a new node, who's children are the 4 clusters, similarly to a quadtree in terrain generation.
4. Split the mesh back into clusters: This may seem weird at first, but it's a critical step as we see later. For now, take note how in the right image we just created a new border going through our group. And how in the graph we now have the two clusters as separate nodes, with both having all original clusters as their children.

import nanite_mesh_5 from './tikz/nanite_mesh_5.jpg';

<figure style={{float:"left"}}>
<img src={nanite_mesh_5} width="350"/>
</figure>

TODO better image

To understand why we split it up again, have a look at the borders of the resulting two clusters. The outer borders, which we weren't allowed to modify during our simplification, are still just as detailed as before. Whereas the new inner border consists of just a single edge and is very simplified. Let's say we start the next iteration, for which we have to collect all the new clusters from all the groups, getting us a mesh with many clusters like you can see on the left (ignoring the red lines for now). We will also have to throw away our group selection, as it changes for each iteration.



## rust-gpu-bindless

## Epilog
