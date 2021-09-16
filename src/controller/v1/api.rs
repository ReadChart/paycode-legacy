extern crate actix_web;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;
extern crate mime;

use actix_web::{
    error,
    HttpResponse,
    post,
    Result,
    web,
    Error,
};

use serde::{
    Serialize, Deserialize,
};
use crate::services::qrcode_services::resolve;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .app_data(
            // Json extractor configuration for this resource.
            web::JsonConfig::default()
                .limit(4096) // Limit request payload size
                .content_type(|mime| {  // <- accept text/plain content type
                    mime.type_() == mime::TEXT && mime.subtype() == mime::PLAIN
                })
                .error_handler(|err, req| {  // <- create custom error response
                    error::InternalError::from_response(
                        err, HttpResponse::Conflict().finish()).into()
                })
        )
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
    Ok(format!("{:?}", res))
}