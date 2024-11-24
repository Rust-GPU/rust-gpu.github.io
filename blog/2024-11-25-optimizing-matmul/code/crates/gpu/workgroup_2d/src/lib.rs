#![no_std]

use settings::Dimensions;
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
    let row = global_id.x as usize;
    let col = global_id.y as usize;

    if row < dimensions.m as usize && col < dimensions.n as usize {
        let mut sum = 0.0;
        for i in 0..dimensions.k as usize {
            sum += a[row * dimensions.k as usize + i] * b[i * dimensions.n as usize + col];
        }
        result[row * dimensions.n as usize + col] = sum;
    }
}
