#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

pub mod api;
pub mod models;
pub mod util;

use actix_web::{middleware::Logger, App, HttpServer};
use api::config::Config;
use api::routes::{login, signup};
use util::create_pool;
use util::Context;

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
            .wrap(Logger::new("%a %{User-Agent}i"))
            .data(context.clone())
            .service(signup)
            .service(login)
    })
    .bind(address)?
    .run()
    .await
}
