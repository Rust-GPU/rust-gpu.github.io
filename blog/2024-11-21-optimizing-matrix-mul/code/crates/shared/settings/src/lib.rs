#![cfg_attr(target_arch = "spirv", no_std)]

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Dimensions {
    pub m: u32,
    pub k: u32,
    pub n: u32,
}

#[cfg(not(target_arch = "spirv"))]
use glam::UVec3;

#[cfg(not(target_arch = "spirv"))]
impl From<UVec3> for Dimensions {
    fn from(uvec: UVec3) -> Self {
        Self {
            m: uvec.x,
            k: uvec.y,
            n: uvec.z,
        }
    }
}

impl Into<(u32, u32, u32)> for Dimensions {
    fn into(self) -> (u32, u32, u32) {
        (self.m, self.k, self.n)
    }
}

impl Dimensions {
    pub fn new(m: u32, k: u32, n: u32) -> Self {
        Self { m, k, n }
    }
}

// Tiling configurations
pub const TILE_SIZE: u32 = 4;
pub const TILE_M: u32 = 4;
pub const TILE_N: u32 = 4;

// Buffer layout information
#[derive(Copy, Clone, Debug)]
pub struct BufferLayout {
    pub binding: u32,
    pub readonly: bool,
}

impl BufferLayout {
    pub const DIMENSIONS: Self = Self {
        binding: 0,
        readonly: true,
    };
    pub const A_MATRIX: Self = Self {
        binding: 1,
        readonly: true,
    };
    pub const B_MATRIX: Self = Self {
        binding: 2,
        readonly: true,
    };
    pub const RESULT: Self = Self {
        binding: 3,
        readonly: false,
    };
}

pub const NUM_BUFFERS: usize = 3;

pub const SHADER_ENTRY_POINT: &str = "matmul";

// Helper functions for index calculations
#[inline]
pub fn get_matrix_index(row: u32, col: u32, stride: u32) -> usize {
    (row * stride + col) as usize
}

pub fn validate_dimensions(a_dims: (u32, u32), b_dims: (u32, u32)) -> bool {
    a_dims.1 == b_dims.0
}

pub fn get_output_dimensions(a_dims: (u32, u32), b_dims: (u32, u32)) -> Option<(u32, u32)> {
    if validate_dimensions(a_dims, b_dims) {
        Some((a_dims.0, b_dims.1))
    } else {
        None
    }
}
