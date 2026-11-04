use crate::api::v1::lucky_wheel::{
    config::Config,
    dto::{api_request, api_response},
    utils::{
        cookie::{create_auth_cookie, delete_auth_cookie},
        jwt::{TokenType, create_token, verify_token},
    },
};
use actix_web::{HttpRequest, HttpResponse, Responder, web};

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

    let cookie = create_auth_cookie(&cfg, &access_token);

    return HttpResponse::Ok()
        .cookie(cookie)
        .json(api_response::Base::success(
            "Login berhasil",
            api_response::SignInData { session_token },
        ));
}

pub async fn sign_out(cfg: web::Data<Config>) -> impl Responder {
    let cookie = delete_auth_cookie(&cfg);

    return HttpResponse::Ok()
        .cookie(cookie)
        .json(api_response::Base::success("Berhasil sign out", ()));
}

pub async fn session_token(cfg: web::Data<Config>, req: HttpRequest) -> impl Responder {
    let cookie = match req.cookie("access_token") {
        Some(c) => c,
        None => {
            return HttpResponse::Unauthorized()
                .json(api_response::Base::failed("Invalid credentials"));
        }
    };
    let access_token = cookie.value();

    let verified = match verify_token(access_token, &cfg.secret_key) {
        Ok(r) => r,
        Err(_) => {
            let cookie = delete_auth_cookie(&cfg);

            return HttpResponse::Unauthorized()
                .cookie(cookie)
                .json(api_response::Base::failed("Invalid credentials"));
        }
    };

    let session_token = create_token(TokenType::SessionToken, &verified.sub, &cfg.secret_key);

    return HttpResponse::Ok().json(api_response::Base::success(
        "Valid token",
        api_response::SignInData { session_token },
    ));
}
