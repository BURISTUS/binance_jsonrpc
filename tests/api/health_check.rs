use crate::{
    helpers::{parse_response, spawn_app},
    types::{HealthCheckResponse, ResponseError},
};

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!({"id": "0", "jsonrpc": "2.0", "method": "health_check", "params": {"random_string": "random"}});

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let response_status = &response.status();

    let parsed_response: HealthCheckResponse = parse_response(response).await.unwrap().unwrap();

    assert!(response_status.is_success());
    assert_eq!(parsed_response.result, "It works");
}

#[tokio::test]
async fn test_create_order_parse_error() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!({"id": "0", "jsonrpc": "2.0", "method": "createorder", "params": {
        "symbol": "test",
        "min_price": "1000.0",
        "max_price": "1100.0",
        "price": "1010.0",
        "quantity": "1.0",
        "side": "BUY",
        "status": "NEW",
    }});

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap().unwrap();

    assert_eq!(parsed_response.error.code, -32601);
    assert_eq!(parsed_response.error.message, "Method not found");
}

#[tokio::test]
async fn test_create_order_invalid_params() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!({"id": "0", "jsonrpc": "2.0", "method": "create_order", "params": {"random_param": "random",}});

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap().unwrap();

    assert_eq!(parsed_response.error.code, -32602);
    assert_eq!(parsed_response.error.message, "Invalid params");
}

// #[tokio::test] дописать передачу параметров и трекать цену в эндпойнте
async fn test_create_order() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!({"id": "0", "jsonrpc": "2.0", "method": "createorder", "params": {
        "symbol": "test",
        "min_price": "1000.0",
        "max_price": "1100.0",
        "price": "1010.0",
        "quantity": "1.0",
        "side": "BUY",
        "status": "NEW",
    }});

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap().unwrap();

    assert_eq!(parsed_response.error.code, -32601);
    assert_eq!(parsed_response.error.message, "Method not found");
}

#[tokio::test]
async fn test_get_order_invalid_params() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!({"id": "0", "jsonrpc": "2.0", "method": "get_order", "params": {"random_param": "random",}});

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap().unwrap();

    assert_eq!(parsed_response.error.code, -32602);
    assert_eq!(parsed_response.error.message, "Invalid params");
}

#[tokio::test]
async fn test_delete_order_invalid_params() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!({"id": "0", "jsonrpc": "2.0", "method": "delete_order", "params": {"random_param": "random",}});

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap().unwrap();

    assert_eq!(parsed_response.error.code, -32602);
    assert_eq!(parsed_response.error.message, "Invalid params");
}

#[tokio::test]
async fn test_jsonrpc_invalid_version() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let json = serde_json::json!({"id": "0", "jsonrpc": "3.0", "method": "get_order", "params": {"random_param": "random",}});

    let response = client
        .post(&format!("{}/api", &app.address))
        .json(&json)
        .send()
        .await
        .expect("Failed to execute request.");

    let parsed_response: ResponseError = parse_response(response).await.unwrap().unwrap();

    assert_eq!(parsed_response.error.code, -32600);
    assert_eq!(parsed_response.error.message, "Invalid Request");
}
