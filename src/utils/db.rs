use crate::{
    binance::api::types::{OrderStatus, OrderType, Side},
    methods::*,
};
use jsonrpc_v2::Error;
use serde::{Deserialize, Serialize};
use sqlx::{types::BigDecimal, PgPool};

/*
    Methods for interacting with
    the databaseusing sqlx.
*/

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderQueryResult {
    pub symbol: String,
    pub order_id: i32,
    pub side: Side,
    pub order_status: OrderStatus,
    pub price: Option<BigDecimal>,
}

pub async fn create_order_record(
    params: OrderCreateParams,
    order_id: i32,
    pg_pool: &PgPool,
) -> Result<i32, jsonrpc_v2::Error> {
    log::info!("Params is: {:?}", params);
    match sqlx::query!(
        r#"
            INSERT INTO orders (symbol, condition_price, price, order_id, quantity, side, order_type, posted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id;
        "#,
        params.symbol,
        params.condition_price,
        params.price,
        order_id,
        params.quantity,
        params.side as Side,
        params.order_type as OrderType,
        chrono::Utc::now()
    )
    .fetch_one(pg_pool)
    .await
    {
        Ok(item) => Ok(item.id),
        Err(e) => {
            log::error!("{}", e);
            Err(Error::Provided { code: (1), message: ("Can't write data to the db")})
        }
    }
}

pub async fn update_order_record(
    response_status: OrderStatus,
    index: i32,
    pg_pool: &PgPool,
) -> Result<(), jsonrpc_v2::Error> {
    match sqlx::query!(
        r#"
            UPDATE orders SET order_status = $1 WHERE id = $2 RETURNING id;
        "#,
        response_status as OrderStatus,
        index as i32,
    )
    .fetch_one(pg_pool)
    .await
    {
        Ok(_) => {
            log::info!("Succesfuly writed to db");
            Ok(())
        }
        Err(e) => {
            log::error!("{}", e);
            Err(Error::Provided {
                code: (1),
                message: ("Can't update db"),
            })
        }
    }
}

pub async fn delete_order_record(
    order_id: i32,
    pg_pool: &PgPool,
) -> Result<String, jsonrpc_v2::Error> {
    match sqlx::query!(
        r#"
            DELETE FROM orders WHERE order_id=$1 RETURNING id;
        "#,
        order_id
    )
    .fetch_one(pg_pool)
    .await
    {
        Ok(id) => Ok(format!("Record with id {:?} was succesfuly delete", id.id)),
        Err(_) => Err(Error::Provided {
            code: (1),
            message: ("Unable to delete record from database. Recod not found"),
        }),
    }
}

pub async fn get_order_record(
    order_id: i32,
    pg_pool: &PgPool,
) -> Result<OrderQueryResult, jsonrpc_v2::Error> {
    match sqlx::query_as!(
            OrderQueryResult,
            r#"
                SELECT order_id AS "order_id!", side AS "side!: Side", symbol AS "symbol!", order_status AS "order_status!: OrderStatus", price AS "price!: Option<BigDecimal>" FROM orders WHERE order_id=$1;
            "#,
            order_id
        )
        .fetch_one(pg_pool)
        .await
        {
            Ok(item) => Ok(OrderQueryResult { symbol: item.symbol, order_id: item.order_id, side: item.side, order_status: item.order_status, price: item.price }),
            Err(e) => {
                log::error!("Unable to select data from the db. Err: {:?}", e);
                Err(Error::Provided { code: (3), message: ("Order with this order id is not found")})
        }
    }
}
