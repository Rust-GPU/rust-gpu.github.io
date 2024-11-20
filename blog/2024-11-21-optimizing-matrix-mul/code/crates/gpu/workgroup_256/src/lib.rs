#![no_std]

use settings::Dimensions;
use spirv_std::glam::UVec3;
use spirv_std::spirv;

#[spirv(compute(threads(256)))]
pub fn matmul(
    #[spirv(global_invocation_id)] global_id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] dimensions: &Dimensions,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] a: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] b: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] result: &mut [f32],
) {
    let index = global_id.x;
    let row = index / dimensions.n;
    let col = index % dimensions.n;

    if index < dimensions.m * dimensions.n {
        let mut sum = 0.0;

        for i in 0..dimensions.k {
            let a_val = a[(row * dimensions.k + i) as usize];
            let b_val = b[(i * dimensions.n + col) as usize];
            sum += a_val * b_val;
        }

        result[(row * dimensions.n + col) as usize] = sum;
    }
}
