use actix_web::{HttpResponse, Responder, web};

use crate::api::v1::lucky_wheel::{
    config::Config,
    dto::{api_request, api_response},
    models::voucher::Voucher,
};

pub async fn create_voucher(
    cfg: web::Data<Config>,
    req: web::Json<api_request::VoucherCreate>,
) -> impl Responder {
    let voucher = Voucher::new(&req.user);

    match cfg.db.vouchers.add(&voucher).await {
        Ok(_r) => HttpResponse::Ok().json(api_response::Base::success(
            "Voucher berhasil dibuat",
            voucher,
        )),
        Err(_) => HttpResponse::InternalServerError()
            .json(api_response::Base::failed("Gagal membuat voucher")),
    }
}

pub async fn get_vouchers(cfg: web::Data<Config>) -> impl Responder {
    match cfg.db.vouchers.get_all().await {
        Ok(r) => HttpResponse::Ok().json(api_response::Base::success("Berhasil memuat data", r)),
        Err(_) => HttpResponse::InternalServerError()
            .json(api_response::Base::failed("Gagal memuat data")),
    }
}
