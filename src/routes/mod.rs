use crate::middleware::auth_middleware::Authorization;
use actix_web::web::{self, ServiceConfig};

pub mod articles;
pub mod auth;
pub mod sources;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/signup").route(web::post().to(auth::signup)))
        .service(web::resource("/login").route(web::post().to(auth::login)))
        .service(
            web::scope("/feed")
                .service(web::resource("/all").route(web::get().to(articles::get_all_feed)))
                .wrap(Authorization),
        )
        .service(web::resource("/refreshToken").route(web::post().to(auth::refresh_token)))
        .service(
            web::scope("/sources")
                .service(
                    web::resource("")
                        .route(web::post().to(sources::subscribe_feed))
                        .route(web::get().to(sources::get_all_feeds)),
                )
                .wrap(Authorization),
        );
}
