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
use actix_web::web::Data;
use r2d2_mysql::mysql::prelude::Queryable;

use crate::errors::resolve_errors::ResolveError;
use crate::services::qrcode_services::resolve;
use crate::structs::app_state::MySQLAppState;
use crate::structs::pay_code::client::PayCodeClientReqBody;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_qr_code_detail);
}

#[post("/getQrCodeDetail")]
pub async fn get_qr_code_detail(req_body: web::Json<PayCodeClientReqBody>, data: Data<MySQLAppState>) -> Result<HttpResponse, ResolveError> {
    let res = resolve(String::from(req_body.get_card_id()), *req_body.get_acc_type())?;
    let pool = &data.get_pool().clone();
    let mut conn = pool.get().expect("Cannot get mysql connection");
    let param = (req_body.get_card_id(), req_body.get_acc_type());

    Ok(HttpResponse::Ok().json(res))
}