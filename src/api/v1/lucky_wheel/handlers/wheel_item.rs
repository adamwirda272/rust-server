use actix_web::{HttpResponse, Responder, web};

use crate::api::v1::lucky_wheel::{
    config::Config,
    dto::{api_request, api_response},
    models::wheel::{WheelItem, WheelItems},
};

pub async fn add_wheel_item(
    cfg: web::Data<Config>,
    req: web::Json<api_request::WheelItemAdd>,
) -> impl Responder {
    let wheel_items = WheelItems::load(&cfg);
    let mut item = WheelItem::create(&req.name, req.weight);

    let result = match wheel_items.add(&item).await {
        Ok(r) => r,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let item_id = result.inserted_id.as_object_id().unwrap();

    item.insert_id(item_id);

    return HttpResponse::Ok().json(api_response::Base::success(
        "Berhasil menambahkan item",
        item,
    ));
}
