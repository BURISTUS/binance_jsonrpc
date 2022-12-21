use bigdecimal::ToPrimitive;
use futures::SinkExt;
use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::{binance::api::model, methods::*};

use super::types::Side;

/*
    Ws connection for price checking.
*/

pub async fn connect_ws(params: &OrderCreateParams, wss_url: String) -> bool {
    let connect_addr = format!("{}{}@ticker", wss_url, params.symbol.to_lowercase());

    let url = url::Url::parse(&connect_addr).unwrap();

    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    log::info!("WebSocket handshake has been successfully completed");
    while let Some(msg) = ws_stream.next().await {
        match msg.unwrap() {
            Message::Text(s) => {
                let parsed: model::DepthStreamWrapper =
                    serde_json::from_str(&s).expect("Can't parse");
                log::info!("{:?}", parsed);
                match params.side {
                    Side::Buy => {
                        if parsed.data.c < params.condition_price.to_f32().unwrap() {
                            return true;
                        }
                    }
                    Side::Sell => {
                        if parsed.data.c > params.condition_price.to_f32().unwrap() {
                            return true;
                        }
                    }
                }
            }
            Message::Ping(ping) => {
                log::info!("Ping: {:?}", ping);
                ws_stream.send(Message::Pong(ping)).await.unwrap();
            }
            Message::Pong(pong) => log::info!("Pong received {:?}", pong),
            Message::Close(close) => {
                log::info!("Close received from binance : {:?}", close);
                return false;
            }
            _ => {
                return false;
            }
        }
    }
    false
}
