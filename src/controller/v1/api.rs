extern crate actix_web;

use actix_web::{post, Result, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service();
}

#[derive(Serialize)]
struct Resp {
    status: u8,
    qrcode: String,
}

#[derive(Deserialize)]
struct Req {
    card_id: String,
    acc_type: u8,
}

#[post("/getQrCodeDetail")]
pub async fn get_qr_code_detail(mut req_payload: web::Payload) -> Result<Resp> {
    match req {}
}