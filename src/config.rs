use std::env;

use mongodb::{Client, options::ClientOptions};

#[derive(Clone)]
pub struct AppConfig {
    pub is_prod: bool,
    pub mongo_client: Client,
}

impl AppConfig {
    pub async fn load() -> Self {
        let app_env = env::var("APP_ENV")
            .expect("APP_ENV must be set")
            .trim()
            .to_lowercase();

        let mongo_uri =
            env::var("MONGODB_URI").expect("Can't find MONGODB_URI");

        let client_options = ClientOptions::parse(&mongo_uri)
            .await
            .expect("Gagal parse MONGODB_URI");

        let mongo_client =
            Client::with_options(client_options).expect("Gagal inisialisasi MongoDB Client");

        Self {
            is_prod: app_env == "prod" || app_env == "production",
            mongo_client,
        }
    }
}