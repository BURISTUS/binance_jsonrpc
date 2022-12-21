use reqwest::Response;
use serde_json;

use super::TelegramConfig;

pub async fn send_message(
    message: String,
    telegram_config: TelegramConfig,
) -> Result<Response, reqwest::Error> {
    // construct request builder
    let client = reqwest::Client::new();
    let json = serde_json::json!({"chat_id": telegram_config.chat_id, "text": message});
    // configure request
    // https://api.telegram.org/bot5838862688:AAGX17S1rBWC19qmIUjDjMQBRuHfmCzItHI/sendMessage
    let resp = client
        .post(telegram_config.bot_url)
        .json(&json)
        .send()
        .await;

    match resp {
        Ok(resp) => Ok(resp),
        Err(err) => {
            log::error!("Failed to send telegram message. Err: {}", err);
            Err(err)
        }
    }
}
