use crate::api::v1::lucky_wheel::{
    config::Config,
    dto::{api_request, api_response},
    utils::jwt::{TokenType, create_token, verify_token},
};
use actix_web::{
    HttpRequest, HttpResponse, Responder,
    cookie::{Cookie, SameSite, time::Duration},
    web,
};

pub async fn sign_in(
    cfg: web::Data<Config>,
    req: web::Json<api_request::SignIn>,
) -> impl Responder {
    if req.username.is_empty() || req.password.is_empty() {
        return HttpResponse::Unauthorized().json(api_response::Base::failed(
            "Mohon input username dan password",
        ));
    }

    let verified: bool = cfg.verify_auth(&req);
    if !verified {
        return HttpResponse::Unauthorized()
            .json(api_response::Base::failed("Username atau password salah"));
    }

    let access_token = create_token(TokenType::AccessToken, &req.username, &cfg.secret_key);
    let session_token = create_token(TokenType::SessionToken, &req.username, &cfg.secret_key);

    let cookie = Cookie::build("access_token", access_token)
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(cfg.is_prod)
        .max_age(Duration::days(30))
        .finish();

    return HttpResponse::Ok()
        .cookie(cookie)
        .json(api_response::Base::success(
            "Login berhasil",
            api_response::SignInData { session_token },
        ));
}

pub async fn sign_out(cfg: web::Data<Config>) -> impl Responder {
    let cookie = Cookie::build("access_token", "")
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(cfg.is_prod)
        .max_age(Duration::ZERO)
        .finish();

    return HttpResponse::Ok()
        .cookie(cookie)
        .json(api_response::Base::success("Berhasil sign out", ()));
}

pub async fn get_session_token(cfg: web::Data<Config>, req: HttpRequest) -> impl Responder {
    let access_token = match req.cookie("access_token") {
        Some(c) => c.value().to_string(),
        None => {
            return HttpResponse::Unauthorized()
                .json(api_response::Base::failed("Invalid credentials"));
        }
    };

    let verified = match verify_token(&access_token, &cfg.secret_key) {
        Ok(r) => r,
        Err(_) => {
            return HttpResponse::Unauthorized()
                .json(api_response::Base::failed("Invalid credentials"));
        }
    };

    let session_token = create_token(TokenType::SessionToken, &verified.sub, &cfg.secret_key);

    return HttpResponse::Ok().json(api_response::Base::success(
        "Valid token",
        api_response::SignInData { session_token },
    ));
}
