use crate::methods;
use crate::utils::configuration::{ApplicationSettings, BinanceConfig};
use crate::utils::TelegramConfig;
use actix_web::{dev::Server, guard, web, App, HttpServer};
use jsonrpc_v2::{Data as JsonRpcData, Server as JsonRpcServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(
        app_configuration: ApplicationSettings,
        db_pool: PgPool,
    ) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            app_configuration.application.host, app_configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            app_configuration.telegram,
            db_pool,
            app_configuration.binance,
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub struct ApplicationBaseUrl(pub String);

fn run(
    listener: TcpListener,
    telegram_config: TelegramConfig,
    db_pool: PgPool,
    binance_config: BinanceConfig,
) -> Result<Server, anyhow::Error> {
    let rpc = JsonRpcServer::new()
        .with_data(JsonRpcData::new(binance_config))
        .with_data(JsonRpcData::new(db_pool))
        .with_data(JsonRpcData::new(telegram_config))
        .with_method("health_check", methods::health_check)
        .with_method("create_order", methods::create_order)
        .with_method("get_order", methods::get_order)
        .with_method("delete_order", methods::delete_order)
        .finish();

    let server = HttpServer::new(move || {
        let rpc = rpc.clone();
        App::new().service(
            web::service("/api")
                .guard(guard::Post())
                .finish(rpc.into_web_service()),
        )
    })
    .listen(listener)?
    .run();

    Ok(server)
    // let server = HttpServer::new(move || {
    //     App::new()
    //         .wrap(TracingLogger::default())
    //         .route("/", web::get().to(json_rpc_handler))
    //         .route("/health_check", web::get().to(health_check))
    //         .app_data(web::JsonConfig::default().limit(4096))
    //         .app_data(base_url.clone())
    //         .app_data(db_pool.clone())
    // })
    // .listen(listener)?
    // .run();
}
