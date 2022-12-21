use super::{types::OrderStatus, utils, BinanceOkResponse, BinanceRequest, BinanceSignedRequest};
use actix_web::http::Method;
use serde::{Deserialize, Serialize};

/*
    Description of the get order request
*/

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRequest {
    pub symbol: String,
    pub order_id: i32,
    pub timestamp: u64,
    pub signature: Option<String>,
}

impl GetRequest {
    pub fn new(symbol: &str, order_id: i32) -> Self {
        let timestamp = utils::timestamp();

        GetRequest {
            symbol: symbol.into(),
            order_id,
            timestamp,
            signature: Default::default(),
        }
    }
}

impl BinanceRequest for GetRequest {
    type Response = GetResponse;

    const ENDPOINT: &'static str = "/api/v3/order";

    const METHOD: Method = Method::GET;
}

impl BinanceSignedRequest for GetRequest {
    fn set_signature_unchecked(&mut self, signature: String) {
        self.signature = Some(signature);
    }

    fn has_signature(&self) -> bool {
        self.signature.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetResponse {
    pub symbol: String,
    pub status: OrderStatus,
    pub order_id: u32,
}

impl GetResponse {
    pub fn new(symbol: &str, status: OrderStatus, order_id: u32) -> Self {
        GetResponse {
            symbol: symbol.into(),
            status,
            order_id,
        }
    }
}

impl<'de> BinanceOkResponse<'de> for GetResponse {}
