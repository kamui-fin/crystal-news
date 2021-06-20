use crate::middleware::auth_middleware::Authorization;
use actix_web::web::{self, ServiceConfig};

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
            web::resource("/refreshToken")
                .wrap(Authorization)
                .route(web::post().to(auth::refresh_token)),
        )
        .service(
            web::scope("/sources")
                .service(web::resource("/subscribe").route(web::post().to(sources::subscribe_feed)))
                .wrap(Authorization),
        );
}
