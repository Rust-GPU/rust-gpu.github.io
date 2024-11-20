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

    let mut sum00: f32 = 0.0;
    let mut sum01: f32 = 0.0;
    let mut sum02: f32 = 0.0;
    let mut sum03: f32 = 0.0;

    for i in 0..dimensions.k as usize {
        let a_elem = a[row * dimensions.k as usize + i];
        sum00 += a_elem * b[i * dimensions.n as usize + col];
        sum01 += a_elem * b[i * dimensions.n as usize + col + 1];
        sum02 += a_elem * b[i * dimensions.n as usize + col + 2];
        sum03 += a_elem * b[i * dimensions.n as usize + col + 3];
    }

    result[row * dimensions.n as usize + col] = sum00;
    result[row * dimensions.n as usize + col + 1] = sum01;
    result[row * dimensions.n as usize + col + 2] = sum02;
    result[row * dimensions.n as usize + col + 3] = sum03;
}
