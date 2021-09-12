extern crate actix_web;

use actix_web::{error, HttpResponse, post, Result, web};

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
struct Resp {
    status: u8,
    qrcode: String,
}

#[derive(Deserialize)]
struct Req {
    card_id: String,
    acc_type: u8,
}

// TODO gaining ecard Id and code type from here and call resolve() to handle communication with pay.dgut.lerongsoft.com and return resolved data
#[post("/getQrCodeDetail")]
pub async fn get_qr_code_detail(mut req_payload: web::Payload) -> Result<Resp> {
    match req {}
}