use actix_web::client::HttpError;
use actix_web::{get, web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}

#[get("/hello")]
pub async fn hello() -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().body("hello from actix-web"))
}
