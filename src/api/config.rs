use dotenv::dotenv;
use envconfig::Envconfig;

use super::error::ApiError;

#[derive(Envconfig, Debug)]
pub struct Config {
    #[envconfig(from = "API_HOST", default = "127.0.0.1")]
    pub api_host: String,
    #[envconfig(from = "API_PORT", default = "3000")]
    pub api_port: u16,
    #[envconfig(from = "DATABASE_URL")]
    pub db_url: String,
}

impl Config {
    pub fn new() -> Result<Self, ApiError> {
        dotenv().ok();
        Config::init_from_env().map_err(|e| ApiError::ConfigError)
    }
}
