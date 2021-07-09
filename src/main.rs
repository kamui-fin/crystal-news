use actix_cors::Cors;
use actix_web::{http, middleware::Logger, App, HttpServer};
use crystal_news::config::init_context;
use crystal_news::routes;
use log::warn;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let context = init_context().await;
    let address = format!("{}:{}", &context.config.api_host, &context.config.api_port);
    pretty_env_logger::init();

    if let Err(e) = sqlx::migrate!("./migrations").run(&context.pool).await {
        warn!("Failed to perform SQLx migrations: {}", e);
    }
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .data(context.clone())
            .configure(routes::config)
    })
    .bind(address)?
    .run()
    .await
}
