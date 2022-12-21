use std::error::Error;

use serde::Deserialize;

pub type RequestResponse<T> = core::result::Result<T, ErrorResponse>;
pub type RequestResult<T> = core::result::Result<RequestResponse<T>, Box<dyn Error>>;

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub code: i16,
    pub msg: String,
}

#[derive(Debug, Deserialize)]
pub struct HealthCheckResponse {
    pub result: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseError {
    pub error: RspError,
}

#[derive(Debug, Deserialize)]
pub struct RspError {
    pub code: i32,
    pub message: String,
}
