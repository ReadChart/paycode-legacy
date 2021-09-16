extern crate actix_web;
extern crate mime;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use actix_web::{
    error,
    Error,
    HttpResponse,
    post,
    Result,
    web,
};
use serde::{
    Deserialize, Serialize,
};

use crate::services::qrcode_services::{RequestData, resolve, ResolveError};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_qr_code_detail);
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

// TODO gaining ecard Id and code type from here and call resolve() to handle communication with pay.dgut.lerongsoft.com and return resolved data
#[post("/getQrCodeDetail")]
pub async fn get_qr_code_detail(req_body: web::Json<Req>) -> Result<String> {
    let res = resolve(String::from(req_body.get_card_id()), *req_body.get_acc_type()).unwrap();
    Ok(serde_json::to_string(&res).unwrap())
}