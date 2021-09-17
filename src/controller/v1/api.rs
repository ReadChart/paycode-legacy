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

use crate::services::qrcode_services::{RequestData, resolve};

#[derive(Debug, Serialize, Display, Error)]
pub enum ResolveError {
    #[display(fmt = "Failed To Decode Result Into Readable Result")]
    DecodeIntoUTF8Error,
    #[display(fmt = "Failed To Decipher Result")]
    DecipherError,
    #[display(fmt = "Unable To Request From Upstream")]
    UpstreamRespError,
    #[display(fmt = "Failed To Read Upstream Response")]
    UpstreamRespUnreadable,
    #[display(fmt = "Failed To Decode Message Into Hex")]
    DecodeIntoHexError,

}

impl error::ResponseError for ResolveError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            ResolveError::DecodeIntoUTF8Error => StatusCode::INTERNAL_SERVER_ERROR,
            ResolveError::DecipherError => StatusCode::INTERNAL_SERVER_ERROR,
            ResolveError::UpstreamRespError => StatusCode::BAD_REQUEST,
            ResolveError::UpstreamRespUnreadable => StatusCode::BAD_REQUEST,
            ResolveError::DecodeIntoHexError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}


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