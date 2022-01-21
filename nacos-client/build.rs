fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    config.default_package_filename("nacos_grpc_service");
    tonic_build::configure()
        // .build_server(false)
        .out_dir("src/grpc/auto")
        .compile_with_config(config, &["proto/nacos_grpc_service.proto"], &["proto"])
        .unwrap();
    Ok(())
}
