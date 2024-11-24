use matmul::MatrixMultiply;
use std::fmt::Display;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;
use tracing::{debug, error, info, instrument, span, trace, warn, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use wgpu::Device;

// Thread-safe global error state for WGPU.
// See https://github.com/gfx-rs/wgpu/issues/2912
static WGPU_ERROR_STATE: OnceLock<Mutex<Option<wgpu::Error>>> = OnceLock::new();

/// Initializes the global error state. Should be called once at startup.
fn init_error_state() {
    WGPU_ERROR_STATE.set(Mutex::new(None)).unwrap();
}

/// Sets an error in the global error state.
fn set_error(error: wgpu::Error) {
    if let Some(state) = WGPU_ERROR_STATE.get() {
        let mut state_lock = state.lock().unwrap();
        *state_lock = Some(error);
    } else {
        panic!("Error state not initialized!");
    }
}

/// Clears the global error state.
fn clear_error() {
    if let Some(state) = WGPU_ERROR_STATE.get() {
        let mut state_lock = state.lock().unwrap();
        *state_lock = None;
    } else {
        panic!("Error state not initialized!");
    }
}

/// Retrieves and clears the last error.
fn take_error() -> Option<wgpu::Error> {
    if let Some(state) = WGPU_ERROR_STATE.get() {
        let mut state_lock = state.lock().unwrap();
        state_lock.take()
    } else {
        panic!("Error state not initialized!");
    }
}

/// Installs a global error handler for the given device.
fn install_error_handler(device: &Device) {
    device.on_uncaptured_error(Box::new(move |error| {
        set_error(error);
    }));
}

fn main() {
    // Initialize the error state.
    init_error_state();

    tracing_subscriber::registry()
        .with(fmt::Layer::default())
        .with(EnvFilter::from_default_env())
        .init();

    let sizes = [
        (2, 2, 2),
        (4, 4, 4),
        (8, 8, 8),
        (16, 16, 16),
        (32, 32, 32),
        (64, 64, 64),
        (128, 128, 128),
        (256, 256, 256),
        (512, 512, 512),
        (1024, 1024, 1024),
        (2048, 2048, 2048),
    ];

    for size in sizes {
        let matmul = matmul::naive::wgpu().unwrap();
        install_error_handler(&matmul.device);
        run_test(matmul, size);
        clear_error();
    }

    for size in sizes {
        let matmul = matmul::workgroup_256::wgpu().unwrap();
        install_error_handler(&matmul.device);
        run_test(matmul, size);
        clear_error();
    }

    for size in sizes {
        let matmul = matmul::workgroup_2d::wgpu().unwrap();
        install_error_handler(&matmul.device);
        run_test(matmul, size);
        clear_error();
    }

    for size in sizes {
        let matmul = matmul::tiling_1d::wgpu().unwrap();
        install_error_handler(&matmul.device);
        run_test(matmul, size);
        clear_error();
    }

    for size in sizes {
        let matmul = matmul::tiling_1d_loop::wgpu().unwrap();
        install_error_handler(&matmul.device);
        run_test(matmul, size);
        clear_error();
    }

    for size in sizes {
        let matmul = matmul::tiling_2d::wgpu().unwrap();
        install_error_handler(&matmul.device);
        run_test(matmul, size);
        clear_error();
    }
}

#[instrument(skip(multiplier, size), fields(algorithm = %multiplier, size=?size))]
fn run_test<T: Display, U: MatrixMultiply<T>>(multiplier: U, size: (u32, u32, u32)) {
    debug!(algorithm = %multiplier, "Starting tests");
    let (m, k, n) = size;

    let span = tracing::span!(Level::DEBUG, "matmul", algorithm = %multiplier, m, k, n);
    let _enter = span.enter();

    trace!("Testing size: {}x{}x{}", m, k, n);

    // Setup phase
    let setup_span = span!(Level::DEBUG, "setup_phase");
    let _setup_enter = setup_span.enter();
    let a: Vec<f32> = (0..m * k).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..k * n).map(|i| i as f32).collect();
    drop(_setup_enter);

    // Compute phase
    let compute_span = span!(Level::DEBUG, "compute_phase");
    let compute_start = Instant::now();
    let _compute_enter = compute_span.enter();
    let result = multiplier.multiply(&a, &b, m, k, n);
    let compute_time = compute_start.elapsed();
    drop(_compute_enter);

    if let Some(error) = take_error() {
        warn!("wgpu error occurred: {:?}", error);
        return;
    }

    if result.is_err() {
        error!("Error during computation: {:?}", result);
        return;
    }

    let result = result.unwrap();

    // Calculate FLOPS
    let flop_span = span!(Level::DEBUG, "calculate_flops");
    let _flop_enter = flop_span.enter();
    let ops = 2.0 * (m * n * k) as f64;
    let flops = ops / compute_time.as_secs_f64() / 1e9;
    info!("Flops: {}", flops);
    drop(_flop_enter);

    // Verification phase
    let verify_span = span!(Level::DEBUG, "verification_phase");
    let _verify_enter = verify_span.enter();
    verify_results(&a, &b, &result, m, k, n);
    drop(_verify_enter);
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
