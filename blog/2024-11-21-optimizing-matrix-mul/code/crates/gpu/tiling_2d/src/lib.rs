#![no_std]

use settings::Dimensions;
use settings::{TILE_M, TILE_N};

use spirv_std::glam::UVec3;
use spirv_std::spirv;

#[spirv(compute(threads(16, 16)))]
pub fn matmul(
    #[spirv(global_invocation_id)] global_id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] dimensions: &Dimensions,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] a: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] b: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] result: &mut [f32],
) {
    let row = (global_id.y * TILE_M as u32) as usize;
    let col = (global_id.x * TILE_N as u32) as usize;

    // Initialize sums array to zeros
    // Note: This is uglier than it needs to be to work around
    // https://github.com/Rust-GPU/rust-gpu/issues/46
    let mut sums: [[f32; TILE_N as usize]; TILE_M as usize] = Default::default();

    // Compute the 2D tile
    for k in 0..dimensions.k as usize {
        for i in 0..TILE_M as usize {
            let a_element = if row + i < dimensions.m as usize {
                a[(row + i) * dimensions.k as usize + k]
            } else {
                0.0
            };

            for j in 0..TILE_N as usize {
                let b_element = if col + j < dimensions.n as usize {
                    b[k * dimensions.n as usize + (col + j as usize)]
                } else {
                    0.0
                };

                sums[i][j] += a_element * b_element;
            }
        }
    }

    // Write results
    for i in 0..TILE_M as usize {
        for j in 0..TILE_N as usize {
            let output_row = row + i as usize;
            let output_col = col + j as usize;

            if output_row < dimensions.m as usize && output_col < dimensions.n as usize {
                result[output_row * dimensions.n as usize + output_col] = sums[i][j];
            }
        }
    }
}
