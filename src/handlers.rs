use actix_web::{web, Responder, HttpResponse, get, post};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use rand::{distributions::Alphanumeric, Rng};
use qrcode::QrCode;
use image::Luma;
use log::{info, error};
use crate::database::UrlRepository;
use crate::models::ShortenRequest;

#[post("/shorten")]
async fn shorten_url<T: UrlRepository>(
    db: web::Data<Arc<T>>,
    req: web::Json<ShortenRequest>,
) -> impl Responder {
    let short_url = req.custom_alias.clone().unwrap_or_else(|| {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect()
    });

    match db.insert_url(&req.original_url, &short_url).await {
        Ok(_) => {
            let qr_code = QrCode::new(&req.original_url).unwrap();
            let image = qr_code.render::<Luma<u8>>().build();
            let file_path = format!("static/{}.png", short_url);
            image.save(&file_path).unwrap();
            info!("Shortened URL created: {} -> {}", req.original_url, short_url);
            HttpResponse::Ok().json(serde_json::json!({"short_url": short_url, "qr_code": file_path}))
        }
        Err(err) => {
            error!("Database insert failed: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{short_url}")]
async fn redirect<T: UrlRepository>(
    db: web::Data<Arc<T>>,
    path: web::Path<String>,
) -> impl Responder {
    let short_url = path.into_inner();
    match db.get_url(&short_url).await {
        Ok(url) => {
            info!("Redirecting to: {}", url.original_url);
            HttpResponse::Found().append_header(("Location", url.original_url)).finish()
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/qrcode/{short_url}")]
async fn generate_qr<T: UrlRepository>(
    db: web::Data<Arc<T>>,
    path: web::Path<String>,
) -> impl Responder {
    let short_url = path.into_inner();
    match db.get_url(&short_url).await {
        Ok(url) => {
            let qr_code = QrCode::new(url.original_url.as_bytes()).unwrap();
            let image = qr_code.render::<Luma<u8>>().build();
            let file_path = format!("static/{}.png", short_url);
            image.save(&file_path).unwrap();
            info!("Generated QR code for: {}", url.original_url);
            HttpResponse::Ok().body(file_path)
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

