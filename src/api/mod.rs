use actix_web::{Scope, web};

use crate::config::AppConfig;

pub mod v1;

pub fn routes(app_cfg: &AppConfig) -> Scope {
    web::scope("/api").service(v1::routes(&app_cfg))
}
