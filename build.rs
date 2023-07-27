fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .file_descriptor_set_path("src/descriptor.bin")
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir("src")
        .compile(
            &[
                "proto/common.proto",
                "proto/security.proto",
                "proto/dfdaemon.proto",
                "proto/manager.proto",
                "proto/scheduler.proto",
            ],
            &["proto/"],
        )?;
    Ok(())
}
