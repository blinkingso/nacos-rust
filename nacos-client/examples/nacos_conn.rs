use chrono::Utc;
use nacos_client::client::cli::GrpcClient;
use nacos_client::client::conn::{GrpcConnection, ServerInfo};
use nacos_common::remote::request::{
    ConfigBatchListenRequest, ConfigListenContext, ConnectionSetupRequest, HealthCheckRequest,
    RpcRequest, ServerCheckRequest,
};
use std::collections::HashMap;
use std::env::set_var;
use std::error::Error;
use tonic::client::Grpc;
use tonic::transport::Channel;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    let mut server_info = ServerInfo {
        server_ip: "127.0.0.1".to_string(),
        server_port: 8848,
        enable_ssl: false,
    };
    let mut grpc_client = GrpcClient {
        connection: None,
        tenant: None,
        client_abilities: Default::default(),
        labels: Default::default(),
        last_active_timestamp: Utc::now().timestamp() as u64,
        server_request_handlers: vec![],
        connection_event_listeners: vec![],
    };
    let conn = grpc_client.connect_to_server(server_info).await?;
    grpc_client.connection = Some(conn);
    let mut config_listen_request = ConfigBatchListenRequest::default();
    let config_context = ConfigListenContext::new(
        "DEFAULT_GROUP".to_string(),
        None,
        String::from("test"),
        None,
    );
    config_listen_request.config_listen_contexts = vec![config_context];
    let conn = grpc_client.connection.as_mut().unwrap();
    conn.request_timeout(config_listen_request, 15000).await?;
    let _ = tokio::signal::ctrl_c().await;
    Ok(())
}
