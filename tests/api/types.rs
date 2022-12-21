use binance_jsonrpc::binance::api::types::{OrderStatus, Side};
use serde::Deserialize;

pub type RequestResponse<T> = core::result::Result<T, ResponseError>;

#[derive(Debug, Deserialize)]
pub struct RpcResponse {
    pub jsonrpc: String,
    pub result: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct RpcGetResponse {
    pub jsonrpc: String,
    pub result: Order,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Order {
    pub symbol: String,
    pub order_id: i32,
    pub side: Side,
    pub order_status: OrderStatus,
    pub price: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct HealthCheckResponse {
    pub result: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseError {
    pub error: RpcError,
}

#[derive(Debug, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}
