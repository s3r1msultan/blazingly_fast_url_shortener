use std::env;
use actix_web::web;
use dotenv::dotenv;
use crate::handlers::{generate_qr, redirect, shorten_url};

pub fn init() {
    init_dotenv();
    init_logging();
}

fn init_dotenv() {
    dotenv().ok();
}

fn init_logging() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
}

pub fn actix_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(shorten_url)
        .service(redirect)
        .service(generate_qr);
}
