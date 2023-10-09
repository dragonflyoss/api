fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .file_descriptor_set_path("src/descriptor.bin")
        .protoc_arg("--experimental_allow_proto3_optional")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Duration", "::prost_wkt_types::Duration")
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
