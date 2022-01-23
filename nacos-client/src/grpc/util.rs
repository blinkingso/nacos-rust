use local_ip_address::local_ip;
use nacos_common::remote::response::RpcResponse;
use nacos_common::{
    error::{NacosError, NacosResult},
    remote::request::RpcRequest,
};
use nacos_proto::grpc::{Metadata, Payload};
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::fmt::format;
use std::net::{IpAddr, Ipv4Addr};
use std::ops::DerefMut;

pub fn convert_request<T>(request: T) -> Payload
where
    T: Serialize + DerefMut<Target = RpcRequest>,
{
    let metadata = Metadata {
        r#type: get_type_name::<T>(),
        client_ip: local_ip()
            .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
            .to_string(),
        headers: request.headers.clone(),
    };

    convert(metadata, &request)
}

fn convert<T: Serialize>(metadata: Metadata, value: &T) -> Payload {
    let json_str = serde_json::to_string(value).unwrap();
    let body = prost_types::Any {
        type_url: "".to_string(),
        value: json_str.into_bytes(),
    };

    Payload {
        metadata: Some(metadata),
        body: Some(body),
    }
}

pub fn parse_request<'de, T>(payload: &'de Payload) -> NacosResult<T>
where
    T: Deserialize<'de> + DerefMut<Target = RpcRequest>,
{
    let _ = check_ty::<T>(payload)?;
    return if let Some(ref body) = payload.body {
        let bytes = body.value.as_slice();
        let mut req: T = serde_json::from_slice::<T>(bytes)?;
        if let Some(ref md) = payload.metadata {
            req.headers = md.headers.clone();
        }
        Ok(req)
    } else {
        Err(NacosError::msg("request data is empty"))
    };
}

pub fn parse_response<'de, T>(payload: &'de Payload) -> NacosResult<T>
where
    T: Deserialize<'de> + DerefMut<Target = RpcResponse>,
{
    let _ = check_ty::<T>(payload)?;
    if let Some(ref body) = payload.body {
        let obj: T = serde_json::from_slice::<T>(body.value.as_slice())?;
        Ok(obj)
    } else {
        return Err(NacosError::msg("payload body is empty"));
    }
}

fn check_ty<T>(payload: &Payload) -> NacosResult<()> {
    let ty = payload
        .metadata
        .as_ref()
        .ok_or(NacosError::msg("metadata is empty"))?
        .r#type
        .clone();
    let ty_ident = get_type_name::<T>();
    if ty_ident != ty {
        return Err(NacosError::msg(format!("Unknown payload type: {}", ty)));
    }

    Ok(())
}

const TYPE_NAME_SPLIT: &'static str = "::";
pub fn get_type_name<T>() -> String {
    let type_name = type_name::<T>();
    if type_name.contains(TYPE_NAME_SPLIT) {
        let (_, r) = type_name.rsplit_once(TYPE_NAME_SPLIT).unwrap();
        return r.to_string();
    }

    type_name.to_string()
}
