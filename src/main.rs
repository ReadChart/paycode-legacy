extern crate actix_web;

use std::sync::Arc;

use actix_web::{App, HttpServer, middleware::Logger, web};
use r2d2::Pool;
use r2d2_mysql::mysql::OptsBuilder;
use r2d2_mysql::MysqlConnectionManager;

use crate::structs::app_state::MySQLAppState;

mod controller;
mod services;
mod errors;
mod structs;

// Main crate
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug, info"); // Setting Environmental Variables
    env_logger::init(); // Initial env_logger
    let mysql_app_data = MySQLAppState::new(
        String::from("paycode_mysql"),
        get_mysql_pool().expect("Failed to get connection"),
    );
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/api")
                .app_data(mysql_app_data)
                .configure(controller::v1::api::config))
            .service(web::scope("/")
                .configure(controller::index::config))
    })
        // Important Note: 127.0.0.1 only allows requests from localhost, while 0.0.0.0 allows all.
        .bind("0.0.0.0:7878")?
        .run()
        .await
}

fn get_mysql_pool() -> Option<Arc<Pool<MysqlConnectionManager>>> {
    let mut opts_builder = OptsBuilder::new();
    opts_builder.db_name(Option::Some("pay_code"));
    opts_builder.bind_address(Option::Some(""));
    opts_builder.tcp_port(3306);
    opts_builder.user(Option::Some(""));
    opts_builder.pass(Option::Some(""));
    let manager = MysqlConnectionManager::new(opts_builder);
    let pool = Arc::new(r2d2::Pool::new(manager).expect("Cannot get mysql connect pool"));
    return Option::Some(pool);
}


#[cfg(test)]
mod tests {
    #[test]
    fn common_test() {
        assert_eq!(2 + 2, 4);
    }
}