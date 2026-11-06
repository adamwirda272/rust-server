use actix_web::{HttpResponse, Responder, web};

use crate::api::v1::lucky_wheel::{
    config::Config,
    dto::{
        api_request::{self, WheelItemUpdate},
        api_response,
    },
    models::wheel::WheelItem,
};

pub async fn add_wheel_item(
    cfg: web::Data<Config>,
    req: web::Json<api_request::WheelItemAdd>,
) -> impl Responder {
    let item = WheelItem::create(&req.name, req.weight);

    match cfg.db.wheel_items.add(&item).await {
        Ok(_r) => HttpResponse::Ok().json(api_response::Base::success(
            "Berhasil menambahkan item",
            item,
        )),
        Err(_) => HttpResponse::InternalServerError()
            .json(api_response::Base::failed("Gagal menambahkan Item")),
    }
}

pub async fn get_wheel_items(cfg: web::Data<Config>) -> impl Responder {
    match cfg.db.wheel_items.get_all().await {
        Ok(r) => return HttpResponse::Ok().json(api_response::Base::success("Berhasil!", r)),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_wheel_item(
    cfg: web::Data<Config>,
    req: web::Json<WheelItemUpdate>,
) -> impl Responder {
    match cfg.db.wheel_items.update(&req).await {
        Ok(r) => {
            HttpResponse::Ok().json(api_response::Base::success("Berhasil update", r.unwrap()))
        }
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
}

pub async fn spin(cfg: web::Data<Config>, req: web::Json<api_request::Spin>) -> impl Responder {
    let mut voucher = match cfg.db.vouchers.find_active(&req.code).await {
        Ok(Some(r)) => r,
        Ok(None) => {
            return HttpResponse::NotFound()
                .json(api_response::Base::failed("Voucher tidak valid"));
        }
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match cfg.db.wheel_items.random().await {
        Ok(Some(spin_result)) => {
            voucher.prize = Some(spin_result.winning_item.name.clone());

            match cfg.db.vouchers.update(&voucher).await {
                Ok(r) => r,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };

            let msg = format!(
                "Selamat anda memenangkan: {}",
                &spin_result.winning_item.name
            );

            HttpResponse::Ok().json(api_response::Base::success(&msg, spin_result))
        }
        Ok(None) => HttpResponse::BadRequest()
            .json(api_response::Base::failed("Wheel kosong atau tidak valid")),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
}
