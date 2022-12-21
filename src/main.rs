use binance_jsonrpc::{startup::Application, utils::configuration::get_configuration};
use env_logger::Env;
use sqlx::PgPool;
use std::fmt::{Debug, Display};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    println!("{:?}", configuration);
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to read data from db");
    println!("{}", &configuration.database.connection_string());
    let application = Application::build(configuration.clone(), connection_pool).await?;
    let application_task = tokio::spawn(application.run_until_stopped());
    tokio::select! {
        o = application_task => report_exit("API", o),
    };

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
