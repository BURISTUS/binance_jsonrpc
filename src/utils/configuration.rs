use serde::Deserialize;

use crate::binance::api::secret_key::SecretKey;

/*
    Reading a config file
    using the config crate.
*/

#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    pub application: ApplicationConfig,
    pub database: DatabaseConfig,
    pub binance: BinanceConfig,
    pub telegram: TelegramConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationConfig {
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BinanceConfig {
    pub url: String,
    pub wss_url: String,
    pub api_key: String,
    pub secret_key: SecretKey,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TelegramConfig {
    pub bot_url: String,
    pub chat_id: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db_name(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_configuration() -> Result<ApplicationSettings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config() {
        let config = get_configuration().unwrap();

        assert_eq!(config.application.base_url, "http://127.0.0.1:");
        assert_eq!(config.application.host, "127.0.0.1");
        assert_eq!(config.application.port, 8000);
        assert_eq!(config.database.host, "localhost");
        assert_eq!(config.database.port, "5432");
        assert_eq!(config.binance.url, "https://testnet.binance.vision");
        assert_eq!(config.telegram.chat_id, "-788100163");
    }
}
