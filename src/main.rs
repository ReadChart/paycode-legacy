#[macro_use]
extern crate rbatis;
extern crate actix_web;

use actix_web::{middleware::Logger, web, App, HttpServer};
use rbatis::rbatis::Rbatis;
use std::sync::Arc;

mod controller;
mod errors;
mod services;
mod structs;

pub const MYSQL_URL: &'static str = "mysql://username:password@ip_addr:port/schema";
// Main crate
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rbatis = Rbatis::new();
    rbatis.link(MYSQL_URL).await?;
    let rbatis = Arc::new(rbatis);
    std::env::set_var("RUST_LOG", "actix_web=debug, info"); // Setting Environmental Variables
    env_logger::init(); // Initial env_logger
    HttpServer::new(move || {
        App::new()
            .data(rbatis.to_owned())
            .wrap(Logger::default())
            .service(web::scope("/api").configure(controller::v1::api::config))
            .service(web::scope("/").configure(controller::index::config))
    })
    // Important Note: 127.0.0.1 only allows requests from localhost, while 0.0.0.0 allows all.
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

#[cfg(test)]
mod tests {

    #[test]
    fn common_test() {}
}
