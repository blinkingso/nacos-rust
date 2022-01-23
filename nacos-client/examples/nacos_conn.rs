use nacos_common::remote::request::{
    ConnectionSetupRequest, HealthCheckRequest, RpcRequest, ServerCheckRequest,
};
use std::collections::HashMap;
use std::env::set_var;
use std::error::Error;
use tonic::client::Grpc;
use tonic::transport::Channel;
use nacos_client::client::conn::{GrpcConnection, health_check, ServerInfo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    let mut req = RpcRequest::default();
    let mut server_info = ServerInfo {
        server_ip: "127.0.0.1".to_string(),
        server_port: 8848
    };
    let connection = GrpcConnection::connect_to_server(server_info.clone()).await?;
    let health_check = health_check(&server_info.server_ip.clone(), server_info.server_port, &connection).await?;
    log::warn!("health check : {:?}", health_check);
    let _ = tokio::signal::ctrl_c().await;
    Ok(())
}

fn create_labels() -> HashMap<String, String> {
    let mut labels = HashMap::new();
    labels.insert(String::from("module"), "config".to_string());
    labels.insert(String::from("source"), "sdk".to_string());
    labels.insert(String::from("taskId"), "0".to_string());
    labels.insert(String::from("AppName"), "unknown".to_string());
    labels.insert(String::from("Vipserver-Tag"), "".to_string());
    labels.insert(String::from("Amory-Tag"), "".to_string());
    labels
}
