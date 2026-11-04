use actix_web::{HttpResponse, Responder, web};
use futures::TryStreamExt; // Wajib di-import untuk .try_collect()
use mongodb::bson::{Document, doc};

use crate::api::v1::lucky_wheel::config::Config;

pub async fn test_db(cfg: web::Data<Config>) -> impl Responder {
    // Tentukan tipe Document
    let collection = cfg.db.collection::<Document>("wheel_items");

    // 1. Jalankan find dengan filter kosong doc!{}
    let cursor = match collection.find(doc! {}).await {
        Ok(cursor) => cursor,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Gagal query ke MongoDB: {}", err));
        }
    };

    // 2. Kumpulkan stream cursor menjadi Vec<Document>
    match cursor.try_collect::<Vec<Document>>().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Gagal membaca stream cursor: {}", err)),
    }
}
