#![no_std]

use settings::Dimensions;
use settings::TILE_SIZE;
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
    let row = global_id.y as usize;
    let col = (global_id.x * TILE_SIZE) as usize;

    if row >= dimensions.m as usize || col >= dimensions.n as usize {
        return;
    }

    // Compute sums for each offset directly
    let mut sums = [0.0; TILE_SIZE as usize];

    for i in 0..dimensions.k as usize {
        let a_elem = a[row * dimensions.k as usize + i];

        for offset in 0..TILE_SIZE as usize {
            if col + offset < dimensions.n as usize {
                let b_elem = b[i * dimensions.n as usize + col + offset];
                sums[offset] += a_elem * b_elem;
            }
        }
    }

    // Write results back
    for offset in 0..TILE_SIZE as usize {
        if col + offset < dimensions.n as usize {
            result[row * dimensions.n as usize + col + offset] = sums[offset];
        }
    }
}
