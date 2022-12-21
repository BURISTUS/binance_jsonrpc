use crate::{
    binance::api::{BinanceRequest, BinanceResponse, BinanceSignedRequest, BinanceSigner},
    utils::configuration::BinanceConfig,
};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

/*
    Binance client to send a request
    and process a response.
*/

pub const HEADER_KEY: &str = "X-MBX-APIKEY";

pub struct BinanceClient {
    client: Client,
    config: BinanceConfig,
}

pub async fn parse_response<R>(response: Response) -> BinanceResponse<R>
where
    R: DeserializeOwned + std::fmt::Debug,
{
    let json = response.json::<serde_json::Value>().await.unwrap();
    log::info!("{:?}", json);
    let response: Result<R, serde_json::Error> = serde_json::from_value(json.clone());
    let parsed_response = match response {
        Ok(response) => Ok(response),
        Err(_) => {
            let error_response = serde_json::from_value(json).unwrap();
            Err(error_response)
        }
    }?;

    Ok(parsed_response)
}

impl BinanceClient {
    pub fn new(config: BinanceConfig) -> Self {
        let binance_client = BinanceClient {
            client: Client::default(),
            config,
        };

        log::info!("Client initialized");
        binance_client
    }

    pub fn sign<T>(&self, mut request: T) -> T
    where
        T: BinanceSignedRequest,
    {
        self.config.secret_key.sign(&mut request);
        request
    }

    pub fn sign_unchecked<T>(&self, mut request: T) -> T
    where
        T: BinanceSignedRequest,
    {
        self.config.secret_key.sign_unchecked(&mut request);
        request
    }

    pub async fn request_future<T>(&self, request: T) -> Response
    where
        T: BinanceRequest,
    {
        let query = request.query();
        let url = format!("{}{}?{}", self.config.url, T::ENDPOINT, query);

        log::info!("Send request to {}", url);

        let request = self
            .client
            .request(T::METHOD, url)
            .header(HEADER_KEY, self.config.api_key.as_str());

        request.send().await.unwrap()
    }
}
