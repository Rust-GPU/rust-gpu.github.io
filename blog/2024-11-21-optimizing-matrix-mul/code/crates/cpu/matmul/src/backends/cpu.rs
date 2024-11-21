use crate::{Cpu, GridComputation, MatrixMultiply};
use glam::UVec3;
use rayon::prelude::*;
use settings::Dimensions;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::future::Future;
use std::sync::Mutex;

/// Run matrix multiplication on the CPU with a single thread.
pub struct SingleThreadedMatMul<T> {
    variant: T,
}

impl<T: Display> Display for SingleThreadedMatMul<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (cpu, single)", self.variant)
    }
}

impl<T> MatrixMultiply<T> for SingleThreadedMatMul<T>
where
    T: Cpu + GridComputation + Display + Send + Sync,
{
    fn new(variant: T) -> impl Future<Output = Self> + Send {
        async move { SingleThreadedMatMul { variant } }
    }

    fn multiply(&self, a: &[f32], b: &[f32], m: u32, k: u32, n: u32) -> Vec<f32> {
        // Initialize the result vector with zeros as that is what the GPU does.
        let mut result = vec![0.0; (m * n) as usize];

        // Retrieve workgroup and dispatch configurations. These tell us how to iterate.
        let workgroup = <T as GridComputation>::workgroup(&self.variant);
        let dispatch = <T as GridComputation>::dispatch_count(&self.variant, m, n);

        // Define dimensions as (m, k, n)
        let dimensions = Dimensions::new(m, k, n);

        // Iterate over the dispatch grid
        for gwx in 0..dispatch.x {
            for gwy in 0..dispatch.y {
                for wx in 0..workgroup.x {
                    for wy in 0..workgroup.y {
                        // Calculate global indices
                        let x = gwx * workgroup.x + wx;
                        let y = gwy * workgroup.y + wy;

                        if x < m && y < n {
                            // Define global id
                            let global_id = UVec3::new(x, y, 1);

                            // Perform the matmul operation for element (x, y). NOTE:
                            // This is the EXACT SAME CODE THAT RUNS ON THE GPU, RUNNING
                            // ON THE CPU. This is the power of rust-gpu.
                            <T as Cpu>::call(
                                &self.variant,
                                global_id,
                                &dimensions,
                                &a,
                                &b,
                                &mut result,
                            );
                        }
                    }
                }
            }
        }

        result
    }
}

/// Run matrix multiplication on the CPU with multiple threads.
pub struct MultiThreadedMatMul<T> {
    variant: T,
}

impl<T: Display> Display for MultiThreadedMatMul<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (cpu, multi)", self.variant)
    }
}

impl<T> MatrixMultiply<T> for MultiThreadedMatMul<T>
where
    T: Cpu + GridComputation + Display + Send + Sync,
{
    fn new(variant: T) -> impl Future<Output = Self> + Send {
        async move { MultiThreadedMatMul { variant } }
    }

    fn multiply(&self, a: &[f32], b: &[f32], m: u32, k: u32, n: u32) -> Vec<f32> {
        // Initialize the result vector with zeros
        let result = vec![0.0; (m * n) as usize];
        let result = Mutex::new(result);

        // Retrieve workgroup and dispatch configurations. These tell us how to iterate.
        let workgroup = <T as GridComputation>::workgroup(&self.variant);
        let dispatch = <T as GridComputation>::dispatch_count(&self.variant, m, n);

        // Define dimensions as (m, k, n)
        let dimensions = Dimensions::new(m, k, n);

        // Precompute all (x, y) indices that need to be processed
        let tasks: Vec<(usize, usize)> = (0..dispatch.x)
            .flat_map(|gwx| {
                (0..dispatch.y).flat_map(move |gwy| {
                    (0..workgroup.x).flat_map(move |wx| {
                        (0..workgroup.y).filter_map(move |wy| {
                            let x = gwx * workgroup.x + wx;
                            let y = gwy * workgroup.y + wy;
                            if x < m && y < n {
                                Some((x as usize, y as usize))
                            } else {
                                None
                            }
                        })
                    })
                })
            })
            .collect();

        // Process each (x, y) pair in parallel
        tasks.par_iter().for_each(|&(x, y)| {
            // Define global_id (adjust z if necessary)
            let global_id = UVec3::new(x as u32, y as u32, 0); // Changed z to 0 for consistency

            // Lock the mutex to get mutable access to the result vector
            let mut result_lock = result.lock().unwrap();

            // Perform the matmul operation for element (x, y)
            <T as Cpu>::call(
                &self.variant,
                global_id,
                &dimensions,
                &a,
                &b,
                &mut result_lock,
            );
        });

        // Extract the result vector from the Mutex
        let result = Mutex::into_inner(result).unwrap();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_threaded_matmul_2x1x1() {
        let m = 2;
        let k = 1;
        let n = 1;

        let a = vec![1.0, 2.0];
        let b = vec![3.0];

        let expected = vec![3.0, 6.0];

        let variant = crate::variants::Isomorphic;
        let matrix_multiplier = futures::executor::block_on(SingleThreadedMatMul::new(variant));

        let result = matrix_multiplier.multiply(&a, &b, m, k, n);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_threaded_matmul_4x4() {
        let m = 4;
        let k = 4;
        let n = 4;

        // Define matrix `a` (4x4) in row-major order
        let a = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ];

        // Define matrix `b` (4x4) in row-major order
        let b = vec![
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        ];

        // Expected result (4x4) after multiplying `a` and `b`
        let expected = vec![
            250.0, 260.0, 270.0, 280.0, 618.0, 644.0, 670.0, 696.0, 986.0, 1028.0, 1070.0, 1112.0,
            1354.0, 1412.0, 1470.0, 1528.0,
        ];

        let variant = crate::variants::Isomorphic;
        let matrix_multiplier = futures::executor::block_on(SingleThreadedMatMul::new(variant));

        let result = matrix_multiplier.multiply(&a, &b, m, k, n);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multithreaded_matmul_2x1x1() {
        let m = 2;
        let k = 1;
        let n = 1;

        let a = vec![1.0, 2.0];
        let b = vec![3.0];

        let expected = vec![3.0, 6.0];

        let variant = crate::variants::Isomorphic;
        let matrix_multiplier = futures::executor::block_on(MultiThreadedMatMul::new(variant));

        let result = matrix_multiplier.multiply(&a, &b, m, k, n);

        assert_eq!(result, expected);
    }
}
