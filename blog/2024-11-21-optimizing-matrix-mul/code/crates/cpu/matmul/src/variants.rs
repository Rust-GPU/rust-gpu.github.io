//! Different implementations of matrix multiplication and the metadata that defines how
//! they run.

use crate::{Cpu, Gpu, GridComputation};
use glam::UVec3;
use settings::Dimensions;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// Naive GPU implementation of matrix multiplication.
pub struct Naive;

impl Display for Naive {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "naive")
    }
}

impl Gpu for Naive {
    fn compiled_shader(&self) -> &[u8] {
        compiled_naive::SHADER_BINARY
    }
}

impl GridComputation for Naive {
    fn workgroup(&self) -> UVec3 {
        UVec3::new(1, 1, 1)
    }

    fn dispatch_count(&self, m: u32, n: u32) -> UVec3 {
        UVec3::new(m * n, 1, 1)
    }
}

/// GPU implementation of matrix multiplication with a workgroup of 256.
pub struct Workgroup256;

impl Display for Workgroup256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "workgroup_256")
    }
}

impl Gpu for Workgroup256 {
    fn compiled_shader(&self) -> &[u8] {
        compiled_workgroup_256::SHADER_BINARY
    }
}

impl GridComputation for Workgroup256 {
    fn workgroup(&self) -> UVec3 {
        UVec3::new(256, 1, 1)
    }

    fn dispatch_count(&self, m: u32, n: u32) -> UVec3 {
        let workgroup = self.workgroup();
        let threads_needed = m * n;
        // This ceil division is needed because Rust handles truncation differently than
        // Typescript/Javascript so we might get 0.
        // We'll also cap the value to a maximum of 65,535 to comply with hardware limits.
        let x = ((threads_needed as f32 / workgroup.x as f32).ceil() as u32).min(65_535);
        UVec3::new(x, 1, 1)
    }
}

/// GPU implementation of matrix multiplication with a two-dimensional workgroup.
pub struct Workgroup2d;

impl Display for Workgroup2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "workgroup_2d")
    }
}

impl Gpu for Workgroup2d {
    fn compiled_shader(&self) -> &[u8] {
        compiled_workgroup_2d::SHADER_BINARY
    }
}

impl GridComputation for Workgroup2d {
    fn workgroup(&self) -> UVec3 {
        UVec3::new(16, 16, 1)
    }

    fn dispatch_count(&self, m: u32, n: u32) -> UVec3 {
        let w = self.workgroup();
        let workgroup_size = w.x + w.y;
        let x = ((m as f32) / (workgroup_size as f32)).ceil() as u32;
        let y = ((n as f32) / (workgroup_size as f32)).ceil() as u32;
        UVec3::new(x, y, 1)
    }
}

/// GPU implementation of matrix multiplication with one-dimensional tiling.
pub struct Tiling1d;

impl Display for Tiling1d {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "tiling_1d")
    }
}

impl Gpu for Tiling1d {
    fn compiled_shader(&self) -> &[u8] {
        compiled_tiling_1d::SHADER_BINARY
    }
}

impl GridComputation for Tiling1d {
    fn workgroup(&self) -> UVec3 {
        UVec3::new(16, 16, 1)
    }

    fn dispatch_count(&self, m: u32, n: u32) -> UVec3 {
        let workgroup = self.workgroup();
        UVec3::new(
            (m + workgroup.x - 1) / workgroup.x,
            (n + workgroup.y - 1) / workgroup.y,
            1,
        )
    }
}

/// GPU implementation of matrix multiplication with one-dimensional tiling (using loops).
pub struct Tiling1dLoop;

impl Display for Tiling1dLoop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "tiling_1d_loop")
    }
}

impl Gpu for Tiling1dLoop {
    fn compiled_shader(&self) -> &[u8] {
        compiled_tiling_1d_loop::SHADER_BINARY
    }
}

impl GridComputation for Tiling1dLoop {
    fn workgroup(&self) -> UVec3 {
        UVec3::new(16, 16, 1)
    }

    fn dispatch_count(&self, m: u32, n: u32) -> UVec3 {
        let workgroup = self.workgroup();
        UVec3::new(
            (m + workgroup.x - 1) / workgroup.x,
            (n + workgroup.y - 1) / workgroup.y,
            1,
        )
    }
}

/// GPU implementation of matrix multiplication with two-dimensional tiling.
pub struct Tiling2dSimd;

impl Display for Tiling2dSimd {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "tiling_2d")
    }
}

impl Gpu for Tiling2dSimd {
    fn compiled_shader(&self) -> &[u8] {
        compiled_tiling_2d::SHADER_BINARY
    }
}

impl GridComputation for Tiling2dSimd {
    fn workgroup(&self) -> UVec3 {
        UVec3::new(16, 16, 1)
    }

    fn dispatch_count(&self, m: u32, n: u32) -> UVec3 {
        let workgroup = self.workgroup();
        UVec3::new(
            (m + workgroup.x - 1) / workgroup.x,
            (n + workgroup.y - 1) / workgroup.y,
            1,
        )
    }
}

/// GPU implementation of matrix multiplication that runs on both the CPU and GPU.
pub struct Isomorphic;

impl Display for Isomorphic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "isomorphic")
    }
}

impl Gpu for Isomorphic {
    fn compiled_shader(&self) -> &[u8] {
        compiled_isomorphic::SHADER_BINARY
    }
}

impl Cpu for Isomorphic {
    fn call(
        &self,
        global_id: UVec3,
        dimensions: &Dimensions,
        a: &[f32],
        b: &[f32],
        results: &mut [f32],
    ) {
        ::isomorphic::matmul(global_id, &dimensions, &a, &b, results);
    }
}

impl GridComputation for Isomorphic {
    fn workgroup(&self) -> UVec3 {
        UVec3::new(16, 16, 1)
    }

    fn dispatch_count(&self, m: u32, n: u32) -> UVec3 {
        let workgroup = self.workgroup();
        UVec3::new(
            (m + workgroup.x - 1) / workgroup.x,
            (n + workgroup.y - 1) / workgroup.y,
            1,
        )
    }
}
