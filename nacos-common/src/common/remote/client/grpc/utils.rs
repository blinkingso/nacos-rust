use local_ip_address::local_ip;
use nacos_api::api::traits::{RequestExt, ResponseExt};
use nacos_core::error::{NacosError, NacosResult};
use nacos_proto::grpc::{Metadata, Payload};
use prost_types::Any;
use std::any::type_name;
use std::net::{IpAddr, Ipv4Addr};

fn convert(metadata: Metadata, body: Vec<u8>) -> Payload {
    Payload {
        metadata: Some(metadata),
        body: Some(Any {
            type_url: "".to_string(),
            value: body,
        }),
    }
}
fn local_ip_address() -> String {
    local_ip()
        .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
        .to_string()
}

/// convert response to payload
pub fn convert_response<Resp>(response: &Resp) -> Payload
where
    Resp: ResponseExt,
{
    let metadata = Metadata {
        r#type: response.ty_name(),
        client_ip: local_ip_address(),
        headers: Default::default(),
    };
    convert(metadata, response.to_vec())
}

/// convert request to payload
pub fn convert_request<Req>(request: &Req) -> Payload
where
    Req: RequestExt,
{
    let metadata = Metadata {
        r#type: request.ty_name(),
        client_ip: local_ip_address(),
        headers: request.headers(),
    };
    convert(metadata, request.to_vec())
}

pub fn parse_response<Resp>(payload: &Payload) -> NacosResult<Resp>
where
    Resp: ResponseExt,
{
    let _ = check_type::<Resp>(payload)?;
    let bytes = payload.body.as_ref().unwrap().value.as_slice();
    Ok(serde_json::from_slice::<Resp>(bytes).unwrap())
}

fn check_type<Ty>(payload: &Payload) -> NacosResult<()> {
    let ty_name = payload.metadata.as_ref().unwrap().r#type.as_str();
    let ty = type_name::<Ty>();
    let ty = if ty.contains("::") {
        ty.rsplit_once("::").unwrap().1
    } else {
        ty
    };
    return if ty != ty_name {
        log::error!("error for expected type: {}, actual is : {}", ty, ty_name);
        Err(NacosError::msg(format!(
            "expected  `{}`, found `{}`",
            ty, ty_name
        )))
    } else {
        Ok(())
    };
}
