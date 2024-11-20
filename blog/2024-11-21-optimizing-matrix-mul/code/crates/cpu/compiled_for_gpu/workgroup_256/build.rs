use spirv_builder::{MetadataPrintout, SpirvBuilder};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpu_crate_path = Path::new("../../../gpu/workgroup_256");

    // Compile the shader crate with SpirvBuilder.
    let result = SpirvBuilder::new(gpu_crate_path, "spirv-unknown-vulkan1.2")
        .print_metadata(MetadataPrintout::Full)
        .build()?;

    // Get the compiled shader as a PathBuf and read its binary content.
    let shader_path = result.module.unwrap_single();
    let shader_binary = fs::read(&shader_path)?;

    // Generate Rust code with a constant holding the shader binary content.
    let shader_binary_literal = shader_binary
        .iter()
        .map(|byte| format!("0x{:02X}", byte))
        .collect::<Vec<_>>()
        .join(", ");
    let generated_code = format!(
        "/// Compiled SPIR-V shader binary\n\
         pub const SHADER_BINARY: &[u8] = &[{}];",
        shader_binary_literal
    );

    // Write this generated code to `OUT_DIR` as `shader_binary.rs`.
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let shader_binary_rs = out_dir.join("shader_binary.rs");
    fs::write(&shader_binary_rs, generated_code)?;

    println!("Generated shader binary constant at {:?}", shader_binary_rs);
    Ok(())
}
