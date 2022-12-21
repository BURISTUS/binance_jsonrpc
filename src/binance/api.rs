use actix_web::http::Method;
use error::BinanceErrorResponse;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub mod create_request;
pub mod delete_request;
pub mod error;
pub mod get_request;
pub mod model;
pub mod secret_key;
pub mod ticker;
pub mod types;
pub mod utils;

pub type BinanceResponse<T> = core::result::Result<T, BinanceErrorResponse>;

pub trait BinanceRequest: Serialize {
    type Response: for<'de> BinanceOkResponse<'de>;

    const ENDPOINT: &'static str;

    const METHOD: Method;

    fn query(&self) -> String
    where
        Self: Sized,
    {
        serde_qs::to_string(self).unwrap()
    }
}

impl<T> BinanceRequest for &T
where
    T: BinanceRequest,
    T: Debug,
{
    type Response = T::Response;

    const ENDPOINT: &'static str = T::ENDPOINT;

    const METHOD: Method = T::METHOD;
}

pub trait BinanceSignedRequest: BinanceRequest {
    fn has_signature(&self) -> bool;

    fn set_signature_unchecked(&mut self, signature: String);

    fn set_signature(&mut self, signature: String) {
        if self.has_signature() {
            panic!("Signature is already set");
        }

        self.set_signature_unchecked(signature);
    }
}

pub trait BinanceOkResponse<'de>: Deserialize<'de> {}

pub trait BinanceSigner {
    fn get_signature_from_query(&self, query: &str) -> String;

    fn sign<T: BinanceSignedRequest>(&self, request: &mut T) {
        let query = request.query();
        let signature = self.get_signature_from_query(&query);
        request.set_signature(signature);
    }

    fn sign_unchecked<T: BinanceSignedRequest>(&self, request: &mut T) {
        let query = request.query();
        let signature = self.get_signature_from_query(&query);
        request.set_signature_unchecked(signature);
    }

    fn sign_query(&self, query: String) -> String {
        if query.contains("signature") {
            panic!("Signature is already set");
        }

        self.sign_query_unchecked(query)
    }

    fn sign_query_unchecked(&self, mut query: String) -> String {
        let signature = self.get_signature_from_query(&query);
        query.push_str("&signature=");
        query.push_str(&signature);
        query
    }
}
