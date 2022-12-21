use crate::methods::*;

use super::{types::*, utils, BinanceOkResponse, BinanceRequest, BinanceSignedRequest};
use actix_web::http::Method;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};


/*
    Description of the create order request.
*/

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub symbol: String,
    pub side: Side,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub quantity: BigDecimal,
    pub quote_order_qty: Option<u32>,
    pub new_client_order_id: Option<u32>,
    pub recv_window: Option<u32>,
    pub timestamp: u64,
    pub time_in_force: Option<TimeInForce>,
    pub price: Option<BigDecimal>,
    pub signature: Option<String>,
}

impl OrderRequest {
    pub fn new(params: &OrderCreateParams) -> Self {
        let timestamp = utils::timestamp();

        OrderRequest {
            symbol: params.symbol.clone(),
            side: params.side.clone(),
            order_type: params.order_type.clone(),
            timestamp,
            quantity: params.quantity.clone(),
            quote_order_qty: Default::default(),
            new_client_order_id: Default::default(),
            recv_window: Default::default(),
            time_in_force: params.time_in_force.clone(),
            price: params.price.clone(),
            signature: Default::default(),
        }
    }
}

impl BinanceRequest for OrderRequest {
    type Response = OrderResponse;

    const ENDPOINT: &'static str = "/api/v3/order";

    const METHOD: Method = Method::POST;
}

impl BinanceSignedRequest for OrderRequest {
    fn set_signature_unchecked(&mut self, signature: String) {
        self.signature = Some(signature);
    }

    fn has_signature(&self) -> bool {
        self.signature.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub symbol: String,
    pub transact_time: u64,
    pub order_id: i32,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: OrderStatus,
}

impl OrderResponse {
    pub fn new(symbol: &str, transaction_time: u64, status: OrderStatus, order_id: i32) -> Self {
        OrderResponse {
            symbol: symbol.into(),
            transact_time: transaction_time,
            status,
            order_id,
            price: Default::default(),
            orig_qty: Default::default(),
            executed_qty: Default::default(),
            cummulative_quote_qty: Default::default(),
        }
    }
}

impl<'de> BinanceOkResponse<'de> for OrderResponse {}
