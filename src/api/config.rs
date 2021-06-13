use dotenv::dotenv;
use envconfig::{Envconfig, Error};

#[derive(Envconfig, Debug, Clone)]
pub struct Config {
    #[envconfig(from = "API_HOST", default = "127.0.0.1")]
    pub api_host: String,
    #[envconfig(from = "API_PORT", default = "3000")]
    pub api_port: u16,
    #[envconfig(from = "DATABASE_URL")]
    pub db_url: String,
    #[envconfig(from = "JWT_SECRET")]
    pub jwt_secret: String,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        dotenv().ok();
        Config::init_from_env()
    }
}
