use std::env;
use dotenv::dotenv;

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

