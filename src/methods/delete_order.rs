use jsonrpc_v2::{Data, Error, Params};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    binance::{
        api::delete_request::{DeleteRequest, DeleteResponse},
        client::{parse_response, BinanceClient},
    },
    utils::{
        configuration::BinanceConfig, delete_order_record, get_order_record, send_message,
        TelegramConfig,
    },
};

/*
    Method for deliting active orders.
*/

#[derive(Debug, Deserialize)]
pub struct OrderCreateParams {
    pub order_id: i32,
}

pub async fn delete_order(
    Params(params): Params<OrderCreateParams>,
    pg_pool: Data<PgPool>,
    binance_config: Data<BinanceConfig>,
    telegram_config: Data<TelegramConfig>,
) -> Result<String, Error> {

    let client = BinanceClient::new(binance_config.clone());
    // Getting order record from db.
    let order = get_order_record(params.order_id, &pg_pool).await?;
    // Using data from order from db, to create new DeleteRequest.
    let delete_request = DeleteRequest::new(order.symbol, params.order_id);
    log::info!("{:?}", delete_request);

    let signed_delete_order_request = client.sign_unchecked(delete_request);
    let res = client.request_future(signed_delete_order_request).await;
    let response: DeleteResponse = match parse_response(res).await {
        Ok(r) => Ok(r),
        Err(err) => {
            log::info!("BinanceError: {:?}", err);
            Err(Error::from(err))
        }
    }?;

    delete_order_record(response.order_id, &pg_pool).await?;

    let order_removed_message = format!(
        "Your order has been deleted! Order id: {}. Symbol: {}",
        response.order_id, response.symbol
    );
    // Tg notification.
    send_message(order_removed_message.clone(), telegram_config.clone())
        .await
        .unwrap();
    Ok(order_removed_message)
}
