use actix_web::cookie::{Cookie, SameSite, time::Duration};

use crate::api::v1::lucky_wheel::config::Config;

pub fn create_auth_cookie<'a>(cfg: &Config, access_token: &'a str) -> Cookie<'a> {
    return Cookie::build("access_token", access_token)
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(cfg.is_prod)
        .max_age(Duration::days(30))
        .finish();
}

pub fn delete_auth_cookie(cfg: &Config) -> Cookie<'static> {
    return Cookie::build("access_token", "")
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(cfg.is_prod)
        .max_age(Duration::ZERO)
        .finish();
}
