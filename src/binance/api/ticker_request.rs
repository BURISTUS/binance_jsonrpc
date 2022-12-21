use super::{BinanceOkResponse, BinanceRequest};
use actix_web::http::Method;
use serde::{Deserialize, Serialize};

/*
    Ticker request to check
    the validity of a pair.
*/

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerRequest {
    pub symbol: String,
}

impl TickerRequest {
    pub fn new(symbol: &str) -> Self {
        TickerRequest {
            symbol: symbol.into(),
        }
    }
}

impl BinanceRequest for TickerRequest {
    type Response = TickerResponse;

    const ENDPOINT: &'static str = "/api/v3/ticker/price";

    const METHOD: Method = Method::GET;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickerResponse {
    pub symbol: String,
}

impl TickerResponse {
    pub fn new(symbol: &str) -> Self {
        TickerResponse {
            symbol: symbol.into(),
        }
    }
}

impl<'de> BinanceOkResponse<'de> for TickerResponse {}
