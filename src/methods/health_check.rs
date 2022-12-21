use jsonrpc_v2::Error;

pub async fn health_check() -> Result<String, Error> {
    Ok("It works".to_string())
}
