use spirv_builder::{Capability, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Change detection for shacer carte
    println!("cargo:rerun-if-changed=./shader/src");
    println!("cargo:rerun-if-changed=./shader/Cargo.toml");
    println!("cargo:rerun-if-changed=./shared/src");
    println!("cargo:rerun-if-changed=./shared/Cargo.toml");

    let mut builder =
        SpirvBuilder::new("./shader", "spirv-unknown-vulkan1.1").capability(Capability::Int16);
    builder.build_script.env_shader_spv_path = Some(true);
    builder.build()?;
    Ok(())
}
