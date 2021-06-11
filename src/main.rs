#[macro_use]
extern crate lazy_static;

pub mod api;
pub mod models;
pub mod util;

use actix_web::{App, HttpServer};
use api::config::Config;
use api::routes::signup;
use util::create_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new().expect("Environmental variables need to be set");
    let pool = create_pool(config.db_url)
        .await
        .expect("Failed to create a database pool");

    HttpServer::new(move || App::new().data(pool.clone()).service(signup))
        .bind(format!("{}:{}", config.api_host, config.api_port))?
        .run()
        .await
}
