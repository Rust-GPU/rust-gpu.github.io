use matmul::MatrixMultiply;
use std::fmt::Display;
use std::time::Instant;
use tracing::{debug, info, instrument, span, trace, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::Layer::default())
        .with(EnvFilter::from_default_env())
        .init();

    let sizes = [
        // Square matrices
        (2, 2, 2),
        (4, 4, 4),
        (8, 8, 8),
        (16, 16, 16),
        (32, 32, 32),
        (64, 64, 64),
        (128, 128, 128),
        // Non-square matrices
        (4, 2, 8),     // A: 4x2, B: 2x8, Result: 4x8
        (8, 4, 2),     // A: 8x4, B: 4x2, Result: 8x2
        (16, 8, 32),   // A: 16x8, B: 8x32, Result: 16x32
        (32, 16, 8),   // A: 32x16, B: 16x8, Result: 32x8
        (64, 32, 128), // A: 64x32, B: 32x128, Result: 64x128
    ];

    run_tests(matmul::naive::wgpu(), &sizes);
    run_tests(matmul::workgroup_256::wgpu(), &sizes);
    run_tests(matmul::workgroup_2d::wgpu(), &sizes);
    run_tests(matmul::tiling_1d::wgpu(), &sizes);
    run_tests(matmul::tiling_2d_simd::wgpu(), &sizes);

    run_tests(matmul::isomorphic::wgpu(), &sizes);
    run_tests(matmul::isomorphic::cpu::single_threaded(), &sizes);
    run_tests(matmul::isomorphic::cpu::multi_threaded(), &sizes);
}

#[instrument(skip(multiplier, sizes), fields(algorithm = %multiplier))]
fn run_tests<T: Display, U: MatrixMultiply<T>>(multiplier: U, sizes: &[(u32, u32, u32)]) {
    debug!(algorithm = %multiplier, "Starting tests");

    for &(m, k, n) in sizes {
        let span = tracing::span!(Level::INFO, "matrix_test", algorithm = %multiplier, m, k, n);
        let _enter = span.enter();

        info!("Testing size: {}x{}x{}", m, k, n);

        // Setup phase
        let setup_span = span!(Level::INFO, "setup_phase");
        let _setup_enter = setup_span.enter();
        let a: Vec<f32> = (0..m * k).map(|i| i as f32).collect();
        let b: Vec<f32> = (0..k * n).map(|i| i as f32).collect();
        drop(_setup_enter);

        // Compute phase
        let compute_span = span!(Level::INFO, "compute_phase");
        let compute_start = Instant::now();
        let _compute_enter = compute_span.enter();
        let result = multiplier.multiply(&a, &b, m, k, n);
        let compute_time = compute_start.elapsed();
        drop(_compute_enter);

        // Calculate GFLOPS
        let gflop_span = span!(Level::INFO, "calculate_gflops");
        let _gflop_enter = gflop_span.enter();
        let ops = 2.0 * (m * n * k) as f64;
        let flops = ops / compute_time.as_secs_f64() / 1e9;
        info!("Flops: {}", flops);
        drop(_gflop_enter);

        // Verification phase
        let verify_span = span!(Level::INFO, "verification_phase");
        let _verify_enter = verify_span.enter();
        verify_results(&a, &b, &result, m, k, n);
        drop(_verify_enter);
    }
}

#[instrument(skip(a, b, result), fields(rows = m, cols = n))]
fn verify_results(a: &[f32], b: &[f32], result: &[f32], m: u32, k: u32, n: u32) {
    let verify_rows = std::cmp::min(m, 2);
    let verify_cols = std::cmp::min(n, 2);

    for i in 0..verify_rows {
        for j in 0..verify_cols {
            let mut expected = 0.0;
            for x in 0..k {
                expected += a[(i * k + x) as usize] * b[(x * n + j) as usize];
            }
            let actual = result[(i * n + j) as usize];
            let diff = (actual - expected).abs();
            assert!(
                diff < 1e-3,
                "Mismatch at [{}, {}]: expected {}, got {}",
                i,
                j,
                expected,
                actual
            );
        }
    }

    trace!("Verification passed");
}
