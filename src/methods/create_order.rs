use crate::{
    binance::{
        api::{
            create_request::{OrderRequest, OrderResponse},
            get_request::{GetRequest, GetResponse},
            ticker::connect_ws,
            types::{OrderStatus, OrderType, Side, TimeInForce},
        },
        client::{parse_response, BinanceClient},
    },
    utils::{
        configuration::{BinanceConfig, TelegramConfig},
        create_order_record, send_message, update_order_record,
    },
};
use bigdecimal::BigDecimal;
use jsonrpc_v2::{Data, Error, Params};
use serde::Deserialize;
use sqlx::PgPool;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct OrderCreateParams {
    pub symbol: String,
    pub condition_price: BigDecimal,
    pub price: Option<BigDecimal>,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: BigDecimal,
    pub side: Side,
    pub order_type: OrderType,
    pub status: OrderStatus,
}

pub async fn create_order(
    Params(params): Params<OrderCreateParams>,
    pool: Data<PgPool>,
    binance_config: Data<BinanceConfig>,
    telegram_config: Data<TelegramConfig>,
) -> Result<String, Error> {
    let client = BinanceClient::new(binance_config.clone());
    let is_condition_met = connect_ws(&params, binance_config.wss_url.clone()).await;

    if !is_condition_met {
        return Err(Error::Provided {
            code: (4),
            message: ("Condition is not completed"),
        });
    }

    let order_request = OrderRequest::new(&params);
    log::info!("{:?}", order_request);

    let signed_order_request = client.sign_unchecked(order_request);
    let res = client.request_future(signed_order_request).await;
    let response: OrderResponse = match parse_response(res).await {
        Ok(r) => Ok(r),
        Err(err) => {
            log::info!("BinanceError: {:?}", err);
            Err(Error::from(err))
        }
    }?;

    let order_posted_message = format!(
        "Your order with sybmol {} has been posted! Order id: {}",
        response.symbol, response.order_id
    );
    send_message(order_posted_message, telegram_config.clone())
        .await
        .unwrap();

    let item = create_order_record(params, response.order_id, &pool).await?;

    loop {
        let get_order = GetRequest::new(&response.symbol, response.order_id);
        let signed_get_request = client.sign_unchecked(get_order);
        let res = client.request_future(signed_get_request.clone()).await;

        log::info!("{:?}", res);

        let response: GetResponse = match parse_response(res).await {
            Ok(r) => Ok(r),
            Err(err) => {
                log::info!("BinanceError: {:?}", err);
                Err(Error::from(err))
            }
        }?;

        if response.status == OrderStatus::Filled {
            update_order_record(response.status, item, &pool).await?;
            let order_complete_message = format!(
                "Your order with symbol {} has been  {:?}! Order id: {}",
                response.symbol, response.status, response.order_id
            );
            send_message(order_complete_message.clone(), telegram_config.clone())
                .await
                .unwrap();

            return Ok(order_complete_message);
        } else if response.status == OrderStatus::Canceled {
            return Ok(format!(
                "Your order: {} with symbols: {} has been {:?}",
                response.order_id, response.symbol, response.status
            ));
        }

        log::info!("{:?}", response);

        tokio::time::sleep(Duration::from_millis(5000)).await;
    }
}
