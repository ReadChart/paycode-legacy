extern crate actix_web;
extern crate derive_more;
extern crate mime;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use actix_web::{error, get, http::header, http::StatusCode, HttpResponse, post, Result, web};
use actix_web::dev::HttpResponseBuilder;
use derive_more::{Display, Error};
use serde::{
    Deserialize, Serialize,
};

use crate::errors::resolve_errors::ResolveError;
use crate::services::qrcode_services::{RequestData, resolve};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_qr_code_detail);
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Resp {
    status: u8,
    qrcode: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Req {
    card_id: String,
    acc_type: u8,
}

impl Req {
    pub fn get_card_id(&self) -> &String {
        &self.card_id
    }
    pub fn get_acc_type(&self) -> &u8 {
        &self.acc_type
    }
}

#[post("/getQrCodeDetail")]
pub async fn get_qr_code_detail(req_body: web::Json<Req>) -> Result<HttpResponse, ResolveError> {
    let res = resolve(String::from(req_body.get_card_id()), *req_body.get_acc_type())?;
    Ok(HttpResponse::Ok().json(res))
}