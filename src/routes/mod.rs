use crate::middleware::auth_middleware::Authorization;
use actix_web::web::{self, ServiceConfig};

pub mod articles;
pub mod auth;
pub mod sources;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/signup").route(web::post().to(auth::signup)))
        .service(web::resource("/login").route(web::post().to(auth::login)))
        .service(
            web::resource("/logout")
                .wrap(Authorization)
                .route(web::delete().to(auth::logout)),
        )
        .service(
            web::resource("/authCheck")
                .wrap(Authorization)
                .route(web::get().to(auth::auth_check)),
        )
        .service(
            web::resource("/refreshToken")
                .wrap(Authorization)
                .route(web::post().to(auth::refresh_token)),
        )
        .service(
            web::resource("/feed")
                .wrap(Authorization)
                .route(web::post().to(articles::get_article_feed)),
        )
        .service(
            web::scope("/sources")
                .service(
                    web::resource("")
                        .route(web::post().to(sources::subscribe_feed))
                        .route(web::get().to(sources::get_all_feeds)),
                )
                .service(
                    web::resource("/{id}")
                        .route(web::delete().to(sources::unsubscribe_feed))
                        .route(web::put().to(sources::update_feed))
                        .route(web::get().to(sources::get_feed)),
                )
                .wrap(Authorization),
        );
}
