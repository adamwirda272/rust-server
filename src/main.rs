use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, middleware::Logger, web};
use dotenvy::dotenv;
use env_logger::Builder;
use log::LevelFilter;

mod api;
mod config;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    // Inisialisasi logger di terminal
    Builder::new()
        .filter_level(LevelFilter::Info)
        .filter_module("actix_web", LevelFilter::Info)
        .init();

    let host = "127.0.0.1";
    let port = 8080;

    let app_cfg = config::AppConfig::load().await;

    println!("Server running on http://{}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .route("/health", web::get().to(health_check))
            .service(api::routes(&app_cfg))
    })
    .bind((host, port))?
    .run()
    .await
}