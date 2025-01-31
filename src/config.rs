use std::sync::Arc;
use actix_web::web;
use dotenv::dotenv;
use crate::database::UrlRepository;
use crate::handlers::{generate_qr, redirect, shorten_url};

pub fn init() {
    init_dotenv();
    init_logging();
}

fn init_dotenv() {
    dotenv().ok();
}

fn init_logging() {
    // env::set_var("RUST_LOG", "info");
    env_logger::init();
}

pub fn config<T: UrlRepository + 'static>(cfg: &mut web::ServiceConfig, db: Arc<T>) {
    cfg.app_data(web::Data::new(db))
        .service(shorten_url)
        .service(redirect)
        .service(generate_qr);
}
