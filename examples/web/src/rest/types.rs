use crate::rest::prelude::ApiResult;
use hyper::{Body, Response};
use serde::Serialize;
use std::collections::HashMap;

pub type RestResult = ApiResult<Response<Body>>;

pub type QueryParams<'a> = HashMap<&'a str, &'a str>;

#[derive(Debug)]
pub struct ReqVariables<'params> {
    pub body: Body,
    pub query_params: QueryParams<'params>,
}

impl<'params> ReqVariables<'params> {
    pub fn new(body: Body, query_params: QueryParams<'params>) -> Self {
        ReqVariables { body, query_params }
    }
}

#[derive(Debug, Serialize)]
pub struct RestResponseData<Data: Serialize> {
    data: Option<Data>,
    error: Option<RestResponseError>,
}

impl<S: Serialize> Default for RestResponseData<S> {
    fn default() -> Self {
        Self {
            data: None,
            error: None,
        }
    }
}

impl<Data: Serialize> RestResponseData<Data> {
    pub fn new(data: Data) -> Self {
        Self {
            data: Some(data),
            ..Default::default()
        }
    }

    pub fn error(err: RestResponseError) -> Self {
        Self {
            error: Some(err),
            ..Default::default()
        }
    }

    pub fn simple_error(message: &'static str) -> Self {
        Self::error(RestResponseError { message })
    }
}

#[derive(Debug, Serialize)]
pub struct RestResponseError {
    message: &'static str,
}

pub const REST_INTERNAL_SERVER_ERROR: RestResponseError = RestResponseError {
    message: "internal server error",
};
