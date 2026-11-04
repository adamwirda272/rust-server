use actix_web::{HttpResponse, Scope, middleware as actix_middleware, web};

use crate::{
    api::v1::lucky_wheel::handlers::{
        auth::{get_session_token, sign_in, sign_out}, test::test_db,
    }, config::AppConfig,
};

pub mod config;
pub mod dto;
pub mod handlers;
mod middleware;
pub mod models;
pub mod utils;

pub fn routes(app_cfg: &AppConfig) -> Scope {
    let config = web::Data::new(config::Config::load(&app_cfg));

    web::scope("lucky-wheel")
        .app_data(config.clone())
        .route("/test", web::get().to(test_db))
        .route("/sign-in", web::post().to(sign_in))
        .route("/sign-out", web::delete().to(sign_out))
        .route("/session-token", web::get().to(get_session_token))
        .service(
            web::scope("/secure")
                .wrap(actix_middleware::from_fn(middleware::check_auth))
                .route("/data", web::get().to(|| async {HttpResponse::Ok().body("Data yang dilindungi")})),
        )
}
