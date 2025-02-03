use actix_web::{web};
use crate::handlers::{home, redirect, shorten_url};

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
            web::resource("/{short_url}")
                .route(web::get().to(redirect)),
        );
}