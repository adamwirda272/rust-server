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
    // 1. Ambil config dari scope
    let cfg = req
        .app_data::<web::Data<Config>>()
        .ok_or_else(|| ErrorUnauthorized("Config not found"))?;

    // 2. Ambil header Authorization
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ErrorUnauthorized("Missing Authorization header"))?;

    // 3. Ekstrak string token setelah "Bearer "
    let token = &auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| ErrorUnauthorized("Invalid Bearer format"))?
        .to_string();

    // 4. Verifikasi JWT token (cukup return Err langsung kalau gagal)
    let claims = match verify_token(&token, &cfg.secret_key) {
        Ok(c) => c,
        Err(_) => return Err(ErrorUnauthorized("Invalid or expired token")),
    };

    // 5. Cek username/sub
    if claims.sub != cfg.username {
        return Err(ErrorUnauthorized("Unauthorized user"));
    }

    // 7. Lanjutkan ke handler
    next.call(req).await
}
