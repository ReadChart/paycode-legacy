//!
//! This is the controller,
//!
extern crate actix_web;
extern crate derive_more;
extern crate mime;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use actix_web::{
    HttpResponse,
    post,
    Result,
    web,
};

use crate::errors::resolve_errors::ResolveError;
use crate::services::qrcode_services::resolve;
use crate::structs::pay_code::client::PayCodeClientReqBody;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_qr_code_detail);
}

#[post("/getQrCodeDetail")]
pub async fn get_qr_code_detail(req_body: web::Json<PayCodeClientReqBody>) -> Result<HttpResponse, ResolveError> {
    let res = resolve(String::from(req_body.get_card_id()), *req_body.get_acc_type())?;
    Ok(HttpResponse::Ok().json(res))
}