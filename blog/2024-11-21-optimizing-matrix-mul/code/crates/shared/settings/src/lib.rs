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

pub const SHADER_ENTRY_POINT: &str = "matmul";
