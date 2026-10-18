use spirv_builder::{Capability, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 셰이더 크레이트는 별도 패키지라 기본적으로는 이 build.rs 가 그 변경을 감지하지 못한다.
    // 아래 경로가 바뀌면 build.rs 를 다시 실행(=셰이더 재컴파일)하도록 명시한다.
    // (디렉터리를 지정하면 cargo 가 하위 파일까지 재귀적으로 감시한다.)
    println!("cargo:rerun-if-changed=./shader/src");
    println!("cargo:rerun-if-changed=./shader/Cargo.toml");
    // 셰이더가 의존하는 공유 크레이트(Params 단일 정의)도 감시한다.
    println!("cargo:rerun-if-changed=./shared/src");
    println!("cargo:rerun-if-changed=./shared/Cargo.toml");

    let mut builder = SpirvBuilder::new("./shader", "spirv-unknown-vulkan1.1")
        .capability(Capability::Int8)
        .uniform_buffer_standard_layout(true);
    builder.build_script.env_shader_spv_path = Some(true);
    builder.build()?;
    Ok(())
}
