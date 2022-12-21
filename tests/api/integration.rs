use binance_jsonrpc::binance::api::types::OrderStatus;

use crate::{
    helpers::{parse_response, spawn_app},
    types::{HealthCheckResponse, ResponseError, RpcGetResponse, RpcResponse},
};

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "health_check",
            "params": {
                "random_string": "random"
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let response_status = &response.status();

    let parsed_response: HealthCheckResponse = parse_response(response).await.unwrap();

    assert!(response_status.is_success());
    assert_eq!(parsed_response.result, "It works");
}

#[tokio::test]
async fn test_create_order_invalid_params() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "create_order",
            "params": {
                "random_param": "random"
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap();

    assert_eq!(parsed_response.error.code, -32602);
    assert_eq!(parsed_response.error.message, "Invalid params");
}

#[tokio::test]
async fn test_create_order_method_not_found() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "createorder",
            "params": {
                "symbol": "test",
                "min_price": "1000.0",
                "max_price": "1100.0",
                "price": "1010.0",
                "quantity": "1.0",
                "side": "BUY",
                "status": "NEW",
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap();

    assert_eq!(parsed_response.error.message, "Method not found");
}

#[tokio::test]
async fn test_get_order_invalid_params() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "get_order",
            "params": {
                "random_param": "random",
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap();

    assert_eq!(parsed_response.error.code, -32602);
    assert_eq!(parsed_response.error.message, "Invalid params");
}

#[tokio::test]
async fn test_delete_order_invalid_params() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "delete_order",
            "params": {
                "random_param": "random"
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap();

    assert_eq!(parsed_response.error.code, -32602);
    assert_eq!(parsed_response.error.message, "Invalid params");
}

#[tokio::test]
async fn test_jsonrpc_invalid_version() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "3.0",
            "method": "get_order",
            "params": {
                "random_param": "random",
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap();

    assert_eq!(parsed_response.error.code, -32600);
    assert_eq!(parsed_response.error.message, "Invalid Request");
}

#[tokio::test]
async fn test_wrong_symbol() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "create_order",
            "params": {
                "symbol": "XRUSDT",
                "condition_price": "1000.0",
                "quantity": "40",
                "side": "BUY",
                "order_type": "MARKET",
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap();

    assert_eq!(parsed_response.error.code, -1121);
    assert_eq!(parsed_response.error.message, "Invalid symbol.");
}

#[tokio::test]
async fn test_create_order() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "create_order",
            "params": {
                "symbol": "XRPUSDT",
                "condition_price": "1000.0",
                "quantity": "40",
                "side": "BUY",
                "order_type": "MARKET"
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: RpcResponse = parse_response(response).await.unwrap();

    assert_eq!(parsed_response.jsonrpc, "2.0");
    assert_eq!(parsed_response.id, "0");
}

#[tokio::test]
async fn test_create_limit_order_without_price() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "create_order",
            "params": {
                "symbol": "XRPUSDT",
                "condition_price": "1000.0",
                "quantity": "40",
                "time_in_force": "GTC",
                "side": "BUY",
                "order_type": "LIMIT"
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap();

    assert_eq!(parsed_response.error.code, -1102);
    assert_eq!(
        parsed_response.error.message,
        "Mandatory parameter 'price' was not sent, was empty/null, or malformed."
    );
}

#[tokio::test]
async fn test_create_limit_order_without_time_in_force() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "create_order",
            "params": {
                "symbol": "XRPUSDT",
                "condition_price": "1000.0",
                "quantity": "40",
                "price": "0.33",
                "side": "BUY",
                "order_type": "LIMIT"
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap();

    assert_eq!(parsed_response.error.code, -1102);
    assert_eq!(
        parsed_response.error.message,
        "Mandatory parameter 'timeInForce' was not sent, was empty/null, or malformed."
    );
}

#[tokio::test]
async fn test_get_order() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "create_order",
            "params": {
                "symbol": "XRPUSDT",
                "condition_price": "1000.0",
                "quantity": "40",
                "side": "BUY",
                "order_type": "MARKET"
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: RpcResponse = parse_response(response).await.unwrap();
    let len = parsed_response.result.len();
    let order_id_str = String::from(&parsed_response.result[len - 6..]);
    let order_id = order_id_str.parse::<i32>().unwrap();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "get_order",
            "params": {
                "order_id": order_id,
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: RpcGetResponse = parse_response(response).await.unwrap();

    assert_eq!(parsed_response.result.symbol, "XRPUSDT");
    assert_eq!(parsed_response.result.order_status, OrderStatus::Filled)
}

#[tokio::test]
async fn test_get_order_with_wrong_order_id() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "get_order",
            "params": {
                "order_id": 1,
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap();

    assert_eq!(
        parsed_response.error.message,
        "Order with this order id is not found"
    );
}

#[tokio::test]
async fn test_delete_order_with_wrong_order_id() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "delete_order",
            "params": {
                "order_id": 1,
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap();

    assert_eq!(
        parsed_response.error.message,
        "Order with this order id is not found"
    );
}

#[tokio::test]
async fn test_create_order_min_notional() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "create_order",
            "params": {
                "symbol": "XRPUSDT",
                "condition_price": "1000.0",
                "quantity": "1",
                "side": "BUY",
                "price": "0.341",
                "time_in_force": "GTC",
                "order_type": "LIMIT"
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap();

    assert_eq!(parsed_response.error.code, -1013);
    assert_eq!(
        parsed_response.error.message,
        "Filter failure: MIN_NOTIONAL"
    );
}

#[tokio::test]
async fn test_create_order_percent_price_by_side() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!(
        {
            "id": "0",
            "jsonrpc": "2.0",
            "method": "create_order",
            "params": {
                "symbol": "XRPUSDT",
                "condition_price": "1000.0",
                "quantity": "40",
                "side": "BUY",
                "price": "100.0",
                "time_in_force": "GTC",
                "order_type": "LIMIT"
            }
        }
    );

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap();

    assert_eq!(parsed_response.error.code, -1013);
    assert_eq!(
        parsed_response.error.message,
        "Filter failure: PERCENT_PRICE_BY_SIDE"
    );
}
