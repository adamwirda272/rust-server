use argon2::{Argon2, PasswordHash, PasswordVerifier};
use mongodb::Database;
use std::env;

use crate::{api::v1::lucky_wheel::dto::api_request, config::AppConfig};

#[derive(Clone)]
pub struct Config {
    pub is_prod: bool,
    pub db: Database,
    pub username: String,
    pub password_hashed: String,
    pub secret_key: String,
}

impl Config {
    pub fn load(app_cfg: &AppConfig) -> Self {
        Self {
            is_prod: app_cfg.is_prod,
            db: app_cfg.mongo_client.database("lucky_wheel"),
            username: env::var("LUCKY_WHEEL_USERNAME").expect("can't found LUCKY_WHEEL_USERNAME"),
            password_hashed: env::var("LUCKY_WHEEL_PASSWORD_HASHED")
                .expect("can't found LUCKY_WHEEL_PASSWORD_HASHED"),
            secret_key: env::var("LUCKY_WHEEL_SECRET_KEY")
                .expect("can't found LUCKY_WHEEL_SECRET_KEY"),
        }
    }

    pub fn verify_auth(&self, req: &api_request::SignIn) -> bool {
        if req.username != self.username {
            return false;
        }
        let parsed_hash = match PasswordHash::new(&self.password_hashed) {
            Ok(o) => o,
            Err(_e) => return false,
        };

        let verified = Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .is_ok();

        if !verified {
            return false;
        }

        return true;
    }
}
