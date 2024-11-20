use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode, Throughput,
};
use matmul::MatrixMultiply;
use rand::Rng;
use std::time::Duration;

const WARMUP_TIME: Duration = Duration::from_secs(2);
const MEASUREMENT_TIME: Duration = Duration::from_secs(5 * 60);
const SAMPLE_SIZE: usize = 10;

/// Matrix sizes to benchmark
const SIZES: &[(u32, u32, u32)] = &[
    // Square matrices
    (2, 2, 2),
    (4, 4, 4),
    (16, 16, 16),
    (64, 64, 64),
    (128, 128, 128),
    (256, 256, 256),
    (512, 512, 512),
    (1024, 1024, 1024),
    (2048, 2048, 2048),
    (4096, 4096, 4096),
    // Non-square matrices
    (4, 2, 8),          // A: 4x2, B: 2x8, Result: 4x8
    (8, 4, 2),          // A: 8x4, B: 4x2, Result: 8x2
    (16, 8, 32),        // A: 16x8, B: 8x32, Result: 16x32
    (32, 16, 8),        // A: 32x16, B: 16x8, Result: 32x8
    (64, 32, 128),      // A: 64x32, B: 32x128, Result: 64x128
    (1024, 512, 2048),  // A: 1024x512, B: 512x2048, Result: 1024x2048
    (2048, 1024, 4096), // A: 2048x1024, B: 1024x4096, Result: 2048x4096
];

fn bench_isomorphic_variants(c: &mut Criterion) {
    // Initialize isomorphic variants
    let multiplier_isomorphic_gpu = matmul::isomorphic::wgpu();
    let multiplier_isomorphic_cpu_single = matmul::isomorphic::cpu::single_threaded();
    let multiplier_isomorphic_cpu_multi = matmul::isomorphic::cpu::multi_threaded();

    for &(m, k, n) in SIZES {
        // Calculate FLOPs for this size
        let flops = 2.0 * (m as f64 * n as f64 * k as f64);

        let mut group = c.benchmark_group(format!("isomorphic_matmul{}x{}x{}", m, k, n));
        group.sampling_mode(SamplingMode::Flat);
        group.warm_up_time(WARMUP_TIME);
        //group.measurement_time(MEASUREMENT_TIME);
        group.sample_size(SAMPLE_SIZE);
        group.throughput(Throughput::Elements(flops as u64));

        // Create matrices for the given size
        let (a, b) = create_test_matrices(m, k, n);

        group.bench_with_input(
            BenchmarkId::new("isomorphic:wgpu", format!("{}x{}x{}", m, k, n)),
            &(m, k, n),
            |bench, &(m, k, n)| {
                bench.iter(|| {
                    black_box(multiplier_isomorphic_gpu.multiply(
                        black_box(&a),
                        black_box(&b),
                        m,
                        k,
                        n,
                    ))
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("isomorphic:cpu:single", format!("{}x{}x{}", m, k, n)),
            &(m, k, n),
            |bench, &(m, k, n)| {
                bench.iter(|| {
                    black_box(multiplier_isomorphic_cpu_single.multiply(
                        black_box(&a),
                        black_box(&b),
                        m,
                        k,
                        n,
                    ))
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("isomorphic:cpu:multi", format!("{}x{}x{}", m, k, n)),
            &(m, k, n),
            |bench, &(m, k, n)| {
                bench.iter(|| {
                    black_box(multiplier_isomorphic_cpu_multi.multiply(
                        black_box(&a),
                        black_box(&b),
                        m,
                        k,
                        n,
                    ))
                });
            },
        );

        group.finish();
    }
}

criterion_group! {
    name = isomorphic;
    config = Criterion::default()
        .with_plots()
        .significance_level(0.01)
        .noise_threshold(0.02);
    targets = bench_isomorphic_variants
}

criterion_main!(isomorphic);

pub fn validate_dimensions(a_dims: (u32, u32), b_dims: (u32, u32)) -> bool {
    a_dims.1 == b_dims.0
}

fn generate_random_matrix(rows: u32, cols: u32) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..rows * cols).map(|_| rng.gen::<f32>()).collect()
}

fn create_test_matrices(m: u32, k: u32, n: u32) -> (Vec<f32>, Vec<f32>) {
    assert!(
        validate_dimensions((m, k), (k, n)),
        "Invalid matrix dimensions"
    );
    (generate_random_matrix(m, k), generate_random_matrix(k, n))
}
