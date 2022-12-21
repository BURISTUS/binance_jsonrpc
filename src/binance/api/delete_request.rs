use super::{utils, BinanceOkResponse, BinanceRequest, BinanceSignedRequest};
use actix_web::http::Method;
use serde::{Deserialize, Serialize};


/*
    Description of the delete active order request.
*/

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRequest {
    pub symbol: String,
    pub order_id: i32,
    pub timestamp: u64,
    pub signature: Option<String>,
}

impl DeleteRequest {
    pub fn new(symbol: String, order_id: i32) -> Self {
        let timestamp = utils::timestamp();

        DeleteRequest {
            symbol,
            order_id,
            timestamp,
            signature: Default::default(),
        }
    }
}

impl BinanceRequest for DeleteRequest {
    type Response = DeleteResponse;

    const ENDPOINT: &'static str = "/api/v3/order";

    const METHOD: Method = Method::DELETE;
}

impl BinanceSignedRequest for DeleteRequest {
    fn set_signature_unchecked(&mut self, signature: String) {
        self.signature = Some(signature);
    }

    fn has_signature(&self) -> bool {
        self.signature.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResponse {
    pub symbol: String,
    pub order_id: i32,
}

impl DeleteResponse {
    pub fn new(symbol: &str, order_id: i32) -> Self {
        DeleteResponse {
            symbol: symbol.into(),
            order_id,
        }
    }
}

impl<'de> BinanceOkResponse<'de> for DeleteResponse {}
