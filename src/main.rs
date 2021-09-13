extern crate actix_web;

use actix_web::{
    App,
    web,
};
use hex::FromHex;

mod controller;
mod services;

// Main crate
#[actix_web::main]
async fn main() -> std::io::Result<()> {}