use crate::api::v1::lucky_wheel::{config::Config, utils::jwt::verify_token};
use actix_web::{
    Error,
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    http::header,
    middleware::Next,
    web,
};

pub async fn check_auth(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let cfg = req
        .app_data::<web::Data<Config>>()
        .ok_or_else(|| ErrorUnauthorized("Config not found"))?;

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ErrorUnauthorized("Missing Authorization header"))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| ErrorUnauthorized("Invalid Bearer format"))?;

    let claims = match verify_token(token, &cfg.auth.secret_key) {
        Ok(c) => c,
        Err(_) => return Err(ErrorUnauthorized("Invalid or expired token")),
    };

    if claims.sub != cfg.auth.username {
        return Err(ErrorUnauthorized("Unauthorized user"));
    }

    next.call(req).await
}
