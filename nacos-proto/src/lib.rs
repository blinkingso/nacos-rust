use crate::grpc::Payload;
use tonic::Response;

pub mod grpc {
    include!("./auto/nacos_grpc_service.rs");
}

#[allow(missing_debug_implementations)]
pub fn log_payload(payload: &Payload) {
    let value = &payload.body.as_ref().unwrap().value;
    let value = String::from_utf8_lossy(value);
    let metadata = payload.metadata.as_ref().unwrap();
    let url = payload.body.as_ref().unwrap().type_url.as_str();
    let message = format!(
        "{:?},body: {{value:{}, type_url: {}}}",
        metadata, value, url
    );
    log::debug!("Payload=> {}", message);
}

pub fn log_response(response: &Response<Payload>) {
    let metadata_map = response.metadata();
    log::debug!("MetadataMap=> {:?}", metadata_map);
    let payload = response.get_ref();
    log_payload(payload);
    let extensions = response.extensions();
    log::debug!("Extensions=> {:?}", extensions);
}

#[test]
fn test_display_payload() {
    let payload = Payload {
        metadata: Some(Metadata {
            r#type: "ConnectionSetupRequest".to_string(),
            client_ip: "".to_string(),
            headers: Default::default(),
        }),
        body: Some(Any {
            type_url: "".to_string(),
            value: String::from("hello world").into_bytes(),
        }),
    };
    println!("{}", display_payload(&payload));
}
