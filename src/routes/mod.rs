use actix_web::web::ServiceConfig;

mod auth;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(auth::login).service(auth::signup).service(auth::logout).service(auth::refresh_token);
}
