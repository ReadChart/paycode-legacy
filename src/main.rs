extern crate actix_web;

use actix_web::{App, HttpServer, middleware::Logger, web};

mod controller;
mod services;
mod errors;
mod structs;

// Main crate
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug, info"); // Setting Environmental Variables
    env_logger::init(); // Initial env_logger
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/api")
                .configure(controller::v1::api::config))
            .service(web::scope("/")
                .configure(controller::index::config))
    })
        .bind("127.0.0.1:7878")?
        .run()
        .await
}


#[cfg(test)]
mod tests {
    #[test]
    fn common_test() {
        assert_eq!(2 + 2, 4);
    }
}