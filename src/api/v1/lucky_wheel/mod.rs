use actix_web::{HttpResponse, Scope, middleware as actix_middleware, web};

use crate::{
    api::v1::lucky_wheel::handlers::{
        auth::{session_token, sign_in, sign_out},
        voucher::{create_voucher, get_vouchers},
        wheel_item::{add_wheel_item, get_wheel_items, spin, update_wheel_item},
    },
    config::AppConfig,
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
        .route("/test", web::post().to(spin))
        .route("/sign-in", web::post().to(sign_in))
        .route("/sign-out", web::delete().to(sign_out))
        .route("/session", web::get().to(session_token))
        .service(
            web::scope("/secure")
                // .wrap(actix_middleware::from_fn(middleware::check_auth))
                .route("/wheel-item", web::get().to(get_wheel_items))
                .route("/wheel-item", web::post().to(add_wheel_item))
                .route("/wheel-item", web::patch().to(update_wheel_item))
                .route("/voucher", web::get().to(get_vouchers))
                .route("/voucher", web::post().to(create_voucher)),
        )
}
