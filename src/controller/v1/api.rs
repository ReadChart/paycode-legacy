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

use crate::services::qrcode_services::{resolve, ResolveError};

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/")]
async fn index() -> Result<&'static str, UserError> {
    serde_json::from_str("213123123123").map_err(|_e| UserError::InternalError)?;
    Ok("success!")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
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
// #[post("/getQrCodeDetail")]
// pub async fn get_qr_code_detail(req_body: web::Json<Req>) -> Result<Resp, ResolveError> {
//     let res = resolve(String::from(req_body.get_card_id()), *req_body.get_acc_type());
//     match res {
//         Ok(res) => Ok(Resp{
//                status: res.status_code,
//                qrcode: String::from(res.qrcode)
//            }),
//         Err(res) => Err(res.map_err),
//     }
// }