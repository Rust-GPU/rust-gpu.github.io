#![allow(opaque_hidden_inferred_bound)]

use glam::UVec3;
use settings::Dimensions;
use std::fmt::Display;
use std::future::Future;
use thiserror::Error;

mod backends;
pub mod variants;

/// Errors that can happen for matrix multiply on the CPU or GPU.
#[derive(Error, Debug)]
pub enum MatrixMultiplyError {
    #[error("Failed to initialize GPU instance")]
    GpuInstanceCreation,
    #[error("Failed to find an appropriate GPU adapter")]
    GpuAdapterRequest,
    #[error("Failed to create GPU device and queue")]
    GpuDeviceCreation,
    #[error("Failed to receive data from the GPU")]
    GpuDataReceive,
    #[error("Mapping GPU buffer failed")]
    GpuBufferMapping,
    #[error("Failed to acquire a lock on the result vector")]
    CpuLockError,
}

/// The trait that defines how to multiply two matrices.
pub trait MatrixMultiply<T>: Display + Sized {
    fn new(variant: T) -> impl Future<Output = Result<Self, MatrixMultiplyError>> + Send;
    fn multiply(
        &self,
        a: &[f32],
        b: &[f32],
        m: u32,
        k: u32,
        n: u32,
    ) -> Result<Vec<f32>, MatrixMultiplyError>;
}

/// Matrix multiplication logic that can be run on the CPU.
pub trait Cpu {
    fn call(
        &self,
        global_id: UVec3,
        dimensions: &Dimensions,
        a: &[f32],
        b: &[f32],
        results: &mut [f32],
    );
}

/// Matrix multiplication logic that can be run on the GPU.
pub trait Gpu {
    fn compiled_shader(&self) -> &[u8];
    fn entry_point(&self) -> &'static str {
        settings::SHADER_ENTRY_POINT
    }
}

/// How to dispatch work.
pub trait GridComputation {
    fn workgroup(&self) -> UVec3;
    fn dispatch_count(&self, m: u32, n: u32) -> UVec3;
}

pub mod naive {
    use super::*;
    use crate::backends::wgpu::MatrixMultiplier;

    pub fn wgpu() -> Result<MatrixMultiplier<variants::Naive>, MatrixMultiplyError> {
        futures::executor::block_on(backends::wgpu::MatrixMultiplier::new(variants::Naive))
    }
}

pub mod workgroup_256 {
    use super::*;
    use crate::backends::wgpu::MatrixMultiplier;

    pub fn wgpu() -> Result<MatrixMultiplier<variants::Workgroup256>, MatrixMultiplyError> {
        futures::executor::block_on(backends::wgpu::MatrixMultiplier::new(
            variants::Workgroup256,
        ))
    }
}

pub mod workgroup_2d {
    use super::*;
    use crate::backends::wgpu::MatrixMultiplier;

    pub fn wgpu() -> Result<MatrixMultiplier<variants::Workgroup2d>, MatrixMultiplyError> {
        futures::executor::block_on(MatrixMultiplier::new(variants::Workgroup2d))
    }
}

pub mod tiling_1d {
    use super::*;
    use crate::backends::wgpu::MatrixMultiplier;

    pub fn wgpu() -> Result<MatrixMultiplier<variants::Tiling1d>, MatrixMultiplyError> {
        futures::executor::block_on(MatrixMultiplier::new(variants::Tiling1d))
    }
}

pub mod tiling_1d_loop {
    use super::*;
    use crate::backends::wgpu::MatrixMultiplier;

    pub fn wgpu() -> Result<MatrixMultiplier<variants::Tiling1dLoop>, MatrixMultiplyError> {
        futures::executor::block_on(MatrixMultiplier::new(variants::Tiling1dLoop))
    }
}

pub mod tiling_2d {
    use super::*;
    use crate::backends::wgpu::MatrixMultiplier;

    pub fn wgpu() -> Result<MatrixMultiplier<variants::Tiling2d>, MatrixMultiplyError> {
        futures::executor::block_on(MatrixMultiplier::new(variants::Tiling2d))
    }
}

pub mod isomorphic {
    use super::*;
    use crate::backends::wgpu::MatrixMultiplier;

    pub fn wgpu() -> Result<MatrixMultiplier<variants::Isomorphic>, MatrixMultiplyError> {
        futures::executor::block_on(MatrixMultiplier::new(variants::Isomorphic))
    }

    pub mod cpu {
        use super::*;
        use crate::backends::cpu::{MultiThreadedMatMul, SingleThreadedMatMul};

        pub fn single_threaded(
        ) -> Result<SingleThreadedMatMul<variants::Isomorphic>, MatrixMultiplyError> {
            futures::executor::block_on(SingleThreadedMatMul::new(variants::Isomorphic))
        }

        pub fn multi_threaded(
        ) -> Result<MultiThreadedMatMul<variants::Isomorphic>, MatrixMultiplyError> {
            futures::executor::block_on(MultiThreadedMatMul::new(variants::Isomorphic))
        }
    }
}
