//!
//! This is the controller,
//!
extern crate actix_web;
extern crate derive_more;
extern crate mime;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use actix_web::{post, web, HttpResponse, Result};
use log::info;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use std::sync::Arc;

use crate::errors::resolve_errors::ResolveError;
use crate::services::qrcode_services::resolve;
use crate::structs::pay_code::client::{PayCodeClientReqBody, PayCodeDto};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_qr_code_detail);
}

#[post("/getQrCodeDetail")]
pub async fn get_qr_code_detail(
    req_body: web::Json<PayCodeClientReqBody>,
    rbatis: web::Data<Arc<Rbatis>>,
) -> Result<HttpResponse, ResolveError> {
    let res = resolve(
        String::from(req_body.get_card_id()),
        *req_body.get_acc_type(),
    )?;
    let mut pay_code = PayCodeDto::default();
    pay_code.card_id = Some(String::from(req_body.get_card_id()));
    pay_code.account_type = Some(req_body.get_acc_type().clone());
    rbatis.save(&pay_code, &[]);
    Ok(HttpResponse::Ok().json(res))
}
