use actix_web::web::{self, ServiceConfig};

pub mod auth;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/signup").route(web::post().to(auth::signup)))
        .service(web::resource("/login").route(web::post().to(auth::login)))
        .service(web::resource("/logout").route(web::delete().to(auth::logout)))
        .service(web::resource("/refreshToken").route(web::post().to(auth::refresh_token)));
}
