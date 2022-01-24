use nacos_common::remote::request::{
    ConnectionSetupRequest, HealthCheckRequest, RpcRequest, ServerCheckRequest,
};
use std::collections::HashMap;
use std::env::set_var;
use std::error::Error;
use chrono::Utc;
use tonic::client::Grpc;
use tonic::transport::Channel;
use nacos_client::client::conn::{GrpcConnection, ServerInfo};
use nacos_client::client::grpc::GrpcClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    let mut server_info = ServerInfo {
        server_ip: "127.0.0.1".to_string(),
        server_port: 8848,
        enable_ssl: false
    };
    let mut grpc_client = GrpcClient {
        connection: None,
        tenant: None,
        client_abilities: Default::default(),
        labels: Default::default(),
        last_active_timestamp: Utc::now().timestamp() as u64,
        server_request_handlers: vec![],
        connection_event_listeners: vec![]
    };
    let conn = grpc_client.connect_to_server(server_info).await?;
    grpc_client.connection = Some(conn);
    let _ = tokio::signal::ctrl_c().await;
    Ok(())
}