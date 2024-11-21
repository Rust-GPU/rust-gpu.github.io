use crate::{Gpu, GridComputation, MatrixMultiply};
use bytemuck;
use futures::channel::oneshot;
use futures::executor::block_on;
use settings::{BufferLayout, Dimensions, SHADER_ENTRY_POINT};
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use tracing::trace;
use wgpu::{self, util::DeviceExt};

/// Matrix multiplication on the GPU using `wgpu`.
pub struct MatrixMultiplier<T> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    variant: T,
}

impl<T: Display> Display for MatrixMultiplier<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (wgpu)", self.variant)
    }
}

impl<T> MatrixMultiply<T> for MatrixMultiplier<T>
where
    T: Gpu + GridComputation + Display + Send,
{
    /// Initializes a new `MatrixMultiplier` with necessary GPU resources.
    async fn new(variant: T) -> Self {
        // Set up WGPU to talk to the system's GPUs and manage rendering or compute tasks.
        let instance = create_instance().await;

        // Find a GPU.
        let adapter = request_adapter(&instance).await;

        // Get access to the GPU and its command system for sending tasks.
        let (device, queue) = request_device_and_queue(&adapter).await;

        // Load the compiled code that we will run on the GPU.
        let shader = create_shader_module(&device, <T as Gpu>::compiled_shader(&variant));

        // Define how the GPU will connect data and resources to the GPU program.
        let bind_group_layout = create_bind_group_layout(&device);

        // Specify how the GPU pipeline organizes its resources and GPU programs.
        let pipeline_layout = create_pipeline_layout(&device, &bind_group_layout);

        // Build the actual GPU pipeline to run the GPU program and manage execution.
        let pipeline = create_compute_pipeline(&device, &pipeline_layout, &shader);

        Self {
            device,
            queue,
            pipeline,
            bind_group_layout,
            variant,
        }
    }

    /// Executes matrix multiplication for given input matrices.
    ///
    /// Uploads the input matrices to the GPU, dispatches the compute shader,
    /// and retrieves the result.
    fn multiply(&self, a: &[f32], b: &[f32], m: u32, k: u32, n: u32) -> Vec<f32> {
        trace!(?a, ?b, "Starting matrix multiplication");

        let result_size = (m * n * std::mem::size_of::<f32>() as u32) as u64;

        // Create a memory buffer on the GPU to store matrix `a`, initialized with data
        // copied from the CPU.
        let a_buffer = create_buffer_init(
            &self.device,
            "Matrix A Buffer",
            a,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        );

        // Create a memory buffer on the GPU to store matrix `b`, initialized with data
        // copied from the CPU.
        let b_buffer = create_buffer_init(
            &self.device,
            "Matrix B Buffer",
            b,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        );

        // Allocate GPU memory for storing the result.
        let result_buffer = create_buffer(
            &self.device,
            "Result Buffer",
            result_size,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        );

        // Create a memory buffer on the GPU to store the dimensions of the matrices,
        // initialized with data copied from the CPU.
        //
        // This is a `uniform` buffer instead of `storage` buffer because the data is
        // the same for all workgroups, it is read-only, and it is small enough to fit
        // in a single buffer (`uniform` buffers are limited to to 64 KB on most GPUs
        // and often less on older GPUs).
        let dimensions = Dimensions::new(m, k, n);
        let dimensions_buffer = create_buffer_init(
            &self.device,
            "Dimensions Buffer",
            &[dimensions],
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        );

        // Group all related buffers for use in the compute pipeline.
        let bind_group = create_bind_group(
            &self.device,
            &self.bind_group_layout,
            &a_buffer,
            &b_buffer,
            &result_buffer,
            &dimensions_buffer,
        );

        // Create a buffer to retrieve computation results back from the GPU.
        let staging_buffer = create_staging_buffer(&self.device, result_size);

        // Set up commands to perform the computation on the GPU.
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Matrix Multiply Encoder"),
            });

        {
            // Define the compute pass, specifying which GPU program to run and what
            // buffers should be involved.
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Matrix Multiply Compute Pass"),
                timestamp_writes: Default::default(),
            });

            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);

            // Dispatch workgroups to perform the matrix multiplication.
            let dispatch_count = <T as GridComputation>::dispatch_count(&self.variant, m, n);
            tracing::trace!("Dispatch counts: {:?}", dispatch_count);
            compute_pass.dispatch_workgroups(dispatch_count.x, dispatch_count.y, dispatch_count.z);
        }

        // Copy the GPU's result into a buffer for CPU access.
        encoder.copy_buffer_to_buffer(&result_buffer, 0, &staging_buffer, 0, result_size);
        self.queue.submit(Some(encoder.finish()));

        // Make the staging buffer's data available to the CPU.
        let slice = staging_buffer.slice(..);
        let (sender, receiver) = oneshot::channel();

        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });

        self.device.poll(wgpu::Maintain::Wait);

        // Wait for the mapping to complete and verify success.
        block_on(receiver)
            .expect("Failed to receive data")
            .expect("Map async failed");

        // Read and convert the result data into a typed vector instead of raw bytes.
        let data = slice.get_mapped_range();
        let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        staging_buffer.unmap();

        trace!(?result, "Matrix multiplication result");
        result
    }
}

/// Creates a new WGPU instance with specified backends.
async fn create_instance() -> wgpu::Instance {
    let backends = wgpu::util::backend_bits_from_env()
        .unwrap_or(wgpu::Backends::VULKAN | wgpu::Backends::METAL);
    wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default(),
        ..Default::default()
    })
}

/// Requests a suitable GPU adapter based on the instance.
async fn request_adapter(instance: &wgpu::Instance) -> wgpu::Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .expect("Failed to find appropriate adapter")
}

/// Requests the GPU device and queue from the adapter.
async fn request_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Matrix Multiply Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device")
}

/// Compiles and creates the shader module from SPIR-V bytes.
fn create_shader_module(device: &wgpu::Device, spirv_bytes: &[u8]) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("SPIR-V Shader Module"),
        source: wgpu::ShaderSource::SpirV(std::borrow::Cow::Borrowed(&pad_and_cast_spirv(
            spirv_bytes,
        ))),
    })
}

/// Defines the bind group layout for the compute pipeline.
fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Matrix Multiply Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: BufferLayout::DIMENSIONS.binding,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: BufferLayout::A_MATRIX.binding,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {
                        read_only: BufferLayout::A_MATRIX.readonly,
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: BufferLayout::B_MATRIX.binding,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {
                        read_only: BufferLayout::B_MATRIX.readonly,
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: BufferLayout::RESULT.binding,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {
                        read_only: BufferLayout::RESULT.readonly,
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    })
}

/// Sets up the pipeline layout using the bind group layout.
fn create_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Matrix Multiply Pipeline Layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    })
}

/// Creates the compute pipeline with the shader and pipeline layout.
fn create_compute_pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
) -> wgpu::ComputePipeline {
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Matrix Multiply Pipeline"),
        layout: Some(pipeline_layout),
        module: shader,
        entry_point: Some(SHADER_ENTRY_POINT),
        compilation_options: Default::default(),
        cache: Default::default(),
    })
}

/// Binds the allocated buffers to the shader's bindings.
fn create_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    a_buffer: &wgpu::Buffer,
    b_buffer: &wgpu::Buffer,
    result_buffer: &wgpu::Buffer,
    dimensions_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Matrix Multiply Bind Group"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: BufferLayout::A_MATRIX.binding,
                resource: a_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: BufferLayout::B_MATRIX.binding,
                resource: b_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: BufferLayout::RESULT.binding,
                resource: result_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: BufferLayout::DIMENSIONS.binding,
                resource: dimensions_buffer.as_entire_binding(),
            },
        ],
    })
}

/// Creates a staging buffer for reading results back to the CPU.
fn create_staging_buffer(device: &wgpu::Device, size: u64) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

/// Initializes a GPU buffer with provided data.
fn create_buffer_init<T: bytemuck::Pod>(
    device: &wgpu::Device,
    label: &str,
    data: &[T],
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::cast_slice(data),
        usage,
    })
}

/// Creates an empty GPU buffer with specified size and usage.
fn create_buffer(
    device: &wgpu::Device,
    label: &str,
    size: u64,
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage,
        mapped_at_creation: false,
    })
}

/// Safely casts a byte slice to a `u32` slice, padding with zeros if necessary.
///
/// Panics if the byte slice cannot be aligned even after padding.
fn pad_and_cast_spirv(bytes: &[u8]) -> Vec<u32> {
    use bytemuck::cast_slice;
    let mut padded = bytes.to_vec();

    // Pad with zeros to make the length a multiple of 4
    while padded.len() % 4 != 0 {
        padded.push(0);
    }

    // Ensure the starting pointer is aligned to 4 bytes
    if padded.as_ptr() as usize % std::mem::align_of::<u32>() != 0 {
        panic!("Shader binary is not 4-byte aligned even after padding.");
    }

    // Safe to cast since we've ensured alignment and length
    cast_slice::<u8, u32>(&padded).to_vec()
}
