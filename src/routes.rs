use actix_web::{web};
use crate::handlers::{home, redirect, shorten_url, url_info};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/")
                .route(web::get().to(home)),
        )
        .service(
            web::resource("/shorten")
                .route(web::post().to(shorten_url)),
        )
        .service(
            web::resource("/u/{short_url}")
                .route(web::get().to(url_info)),
        )
        .service(
            web::resource("/{short_url}")
                .route(web::get().to(redirect)),
        );
}