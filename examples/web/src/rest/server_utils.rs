use crate::error::StdResult;
use crate::rest::prelude::*;
use serde::{de, ser};

pub async fn deserialize_request_body<T>(req_body: Body) -> StdResult<T>
where
    T: de::DeserializeOwned,
{
    let body_bytes = hyper::body::to_bytes(req_body).await?;
    serde_json::from_slice(&body_bytes).map_err(From::from)
}

pub fn serialize_response<T>(res: Response<T>) -> RestResult
where
    T: ser::Serialize,
{
    let (parts, body) = res.into_parts();
    let body = serde_json::to_vec(&body)?;
    Ok(Response::from_parts(parts, Body::from(body)))
}

pub fn get_query_params(req_query: Option<&str>) -> QueryParams<'_> {
    req_query
        .map(|query| {
            query
                .split('&')
                .into_iter()
                .filter_map(|param| {
                    let mut parts = param.split('=');
                    if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                        Some((key, value))
                    } else {
                        None
                    }
                })
                .collect::<QueryParams<'_>>()
        })
        .unwrap_or_default()
}

pub fn create_not_found_err_json_response(message: &'static str) -> RestResult {
    create_err_json_response(StatusCode::NOT_FOUND, message)
}

pub fn create_err_json_response(status: StatusCode, message: &'static str) -> RestResult {
    create_json_response::<serde_json::Value>(status, RestResponseData::simple_error(message))
}

pub fn create_ok_json_response<Data: ser::Serialize>(body: Data) -> RestResult {
    create_json_response(StatusCode::OK, RestResponseData::new(body))
}

pub fn create_json_response<Data: ser::Serialize>(
    status: StatusCode,
    body: RestResponseData<Data>,
) -> RestResult {
    let res = Response::builder()
        .status(status)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        )
        .body(body)?;
    serialize_response(res)
}
