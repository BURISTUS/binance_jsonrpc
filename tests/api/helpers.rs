use binance_jsonrpc::{
    server::Application,
    utils::{configuration::get_configuration, DatabaseConfig},
};
use reqwest::Response;
use serde::de::DeserializeOwned;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use crate::types::RequestResponse;

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
    pub api_client: reqwest::Client,
}

pub async fn spawn_app() -> TestApp {
    let mut configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.application.port = 0;
        c
    };

    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_db(&configuration.database).await;

    let application = Application::build(configuration.clone(), connection_pool.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();

    let _ = tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        db_pool: connection_pool,
        api_client: client,
    };

    test_app
}

pub async fn configure_db(config: &DatabaseConfig) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db_name())
        .await
        .expect("Failed to connect to db");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, &config.database_name).as_str())
        .await
        .expect("Failed to create db");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to db");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

pub async fn parse_response<R>(response: Response) -> RequestResponse<R>
where
    R: DeserializeOwned + std::fmt::Debug,
{
    let json = response.json::<serde_json::Value>().await.unwrap();
    println!("{:?}", json);

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
