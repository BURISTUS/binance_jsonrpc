use jsonrpc_v2::{Data, Error, Params};
use serde::Deserialize;
use sqlx::PgPool;

use crate::utils::{get_order_record, OrderQueryResult};

/*
    Method for returning created orders
*/

#[derive(Debug, Deserialize)]
pub struct OrderDeleteParams {
    pub order_id: i32,
}

pub async fn get_order(
    Params(params): Params<OrderDeleteParams>,
    pg_pool: Data<PgPool>,
) -> Result<OrderQueryResult, Error> {
    let order = get_order_record(params.order_id, &pg_pool).await?;
    log::info!("{:?}", order);
    Ok(order)
}
