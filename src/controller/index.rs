use actix_web::{HttpResponse, web, get};
use actix_web::client::HttpError;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}

#[get("/hello")]
pub async fn hello() -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().body("hello from actix-web"))
}