use actix_web::{Scope, web};

use crate::config::AppConfig;

pub mod lucky_wheel;

pub fn routes(app_cfg: &AppConfig) -> Scope {
    web::scope("/v1")
        .service(lucky_wheel::routes(&app_cfg))
}