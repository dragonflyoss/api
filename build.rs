use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let descriptor_path = out_dir.join("descriptor.bin");

    tonic_build::configure()
        .file_descriptor_set_path(descriptor_path)
        .protoc_arg("--experimental_allow_proto3_optional")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "scheduler.v2.AnnouncePeerRequest.request",
            "#[allow(clippy::large_enum_variant)]",
        )
        .type_attribute(
            "scheduler.v2.AnnounceCachePeerRequest.request",
            "#[allow(clippy::large_enum_variant)]",
        )
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Duration", "::prost_wkt_types::Duration")
        .compile(
            &[
                "proto/common.proto",
                "proto/errordetails.proto",
                "proto/dfdaemon.proto",
                "proto/manager.proto",
                "proto/scheduler.proto",
            ],
            &["proto/"],
        )?;
    Ok(())
}
