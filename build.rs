fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir("src")
        .compile(
            &[
                "proto/common.proto",
                "proto/security.proto",
                "proto/errordetails.proto",
                "proto/dfdaemon.proto",
                "proto/manager.proto",
                "proto/scheduler.proto",
            ],
            &["proto/"],
        )?;
    Ok(())
}
