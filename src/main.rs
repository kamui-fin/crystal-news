#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

pub mod config;
pub mod error;
pub mod models;
pub mod routes;
pub mod util;

use actix_web::{middleware::Logger, App, HttpServer};
use config::{Config, Context};
use util::create_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new().expect("Environmental variables need to be set");
    pretty_env_logger::init();

    let address = format!("{}:{}", &config.api_host, &config.api_port);

    let pool = create_pool(config.db_url.clone())
        .await
        .expect("Failed to create a database pool");

    if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
        eprintln!("Failed to perform SQLx migrations: {}", e);
    }

    let context = Context { pool, config };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(context.clone())
            .configure(routes::config)
    })
    .bind(address)?
    .run()
    .await
}
