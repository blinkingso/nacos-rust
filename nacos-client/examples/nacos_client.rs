use futures_core::Stream;
use nacos_common::grpc_client::GrpcClient;
use nacos_common::remote::request::{
    ConnectionSetupRequest, HealthCheckRequest, RpcRequest, ServerCheckRequest,
};
use nacos_common::utils;
use std::collections::HashMap;
use std::env::set_var;
use std::error::Error;
use tonic::client::Grpc;
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    set_var("RUST_LOG", "debug");
    env_logger::init();
    let mut req = RpcRequest::default();
    // req.set_request_id("".to_string());
    req.put_header("accessToken".to_string(), "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJuYWNvcyIsImV4cCI6MTY0MjE3Mzc2Nn0.xQccuBO50TTW2SXjdiGrBmtVw_fKu4c1NDr0-UBmuW8".to_string());
    let channel = Channel::builder("http://127.0.0.1:9848".parse().unwrap())
        .connect()
        .await?;
    let mut client = request_client::RequestClient::new(channel.clone());
    let server_check_request = ServerCheckRequest::new(req.clone());
    let response = client
        .request(tonic::Request::new(utils::convert_request(
            &server_check_request,
        )))
        .await?;
    if let Some(ref data) = response.get_ref().body {
        let msg = String::from_utf8_lossy(data.value.as_slice());
        log::info!("got body is : {}", msg);
    } else {
        log::warn!("body is none");
    }
    let mut bi_stream_client =
        bi_request_stream_client::BiRequestStreamClient::new(channel.clone());
    let setup_request =
        ConnectionSetupRequest::new(req.clone(), "Nacos-Rust-Client:0.0.1", "", create_labels());
    log::info!(
        "setup_request: {}",
        serde_json::to_string(&setup_request).unwrap()
    );
    let payload = utils::convert_request(&setup_request);
    log::warn!("csr: {:?}", payload);
    let mut request = tonic::Request::new(futures_util::stream::iter(vec![payload]));
    request.set_timeout(std::time::Duration::from_secs(5));
    let mut response = bi_stream_client.request_bi_stream(request).await?;
    let headers = response.metadata().clone();
    let payload = response.get_mut();
    log::info!("metadata map: {:?}", headers);
    if let Some(ref data) = payload.message().await? {
        let data = data.body.as_ref().unwrap();
        let msg = String::from_utf8_lossy(data.value.as_slice());
        log::info!("got body is : {}", msg);
    } else {
        log::warn!("body is none");
    }

    let health_check = HealthCheckRequest { request: req };
    let response = client
        .request(tonic::Request::new(utils::convert_request(&health_check)))
        .await?;
    let payload = response.get_ref();
    if let Some(ref data) = payload.body {
        let msg = String::from_utf8_lossy(data.value.as_slice());
        log::info!("got body is : {}", msg);
    } else {
        log::warn!("body is none");
    }

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
