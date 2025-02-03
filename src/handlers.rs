use actix_files::NamedFile;
use std::sync::Arc;
use actix_web::{web, HttpResponse, Responder};
use log::{info, error, debug};

use crate::database::UrlRepository;
use crate::hash::{base62_encode, fnv1a_hash, generate_leet_variations};
use crate::models::{ShortenRequest, DB};
use crate::validation::is_valid_url;

pub async fn shorten_url(
    db: web::Data<DB>,
    req: web::Json<ShortenRequest>,
) -> impl Responder {

    if !is_valid_url(&req.original_url) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid URL"
        }));
    }

    let short_url = if let Some(alias) = &req.custom_alias {
        if alias.len() < 3 {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Alias is too short"
            }));
        }
        if alias.chars().any(|c| !c.is_alphanumeric()) {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Alias contains invalid characters"
            }));
        }
        alias
    } else {
        &base62_encode(fnv1a_hash(&req.original_url))
    };

    if let Ok(Some(_)) = db.read().await.get_url(&short_url).await {
        let mut suggestions = Vec::new();
        let variations = generate_leet_variations(&short_url);

        for variation in variations {
            if suggestions.len() == 5 {
                break;
            }
            if let Ok(None) = db.read().await.get_url(&variation).await {
                suggestions.push(variation);
            }
        }

        return HttpResponse::Conflict().json(serde_json::json!({
            "error": "Alias already taken",
            "suggestions": suggestions
        }));
    }

    match db.read().await.insert_url(&req.original_url, &short_url).await {
        Ok(_) => {
            info!("{} -> {}", req.original_url, short_url);
            HttpResponse::Ok().json(serde_json::json!({
                "short_url": short_url
            }))
        }
        Err(err) => {
            error!("Database error: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}


pub async fn redirect(
    db: web::Data<DB>,
    path: web::Path<String>,
) -> impl Responder {
    let short_url = path.into_inner();
    match db.read().await.get_url(&short_url).await {
        Ok(url) => {

            let original_url = if let Some(url) = url {
                url.original_url
            } else {
                return HttpResponse::NotFound().finish();
            };

            HttpResponse::Found().append_header(("Location", original_url)).finish()
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn home() -> impl Responder {
    NamedFile::open("./static/index.html").map_err(|e| {
        error!("Error serving index.html: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })
}

pub async fn url_info(
    db: web::Data<DB>,
    path: web::Path<String>,
) -> impl Responder {
    let short_url = path.into_inner();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    match db.read().await.get_url(&short_url).await {
        Ok(Some(url)) => {
            if url.expiration_date <= now {
                return HttpResponse::Gone().json(serde_json::json!({
                    "error": "URL has expired"
                }));
            }

            HttpResponse::Ok().json(serde_json::json!({
                "short_url": short_url,
                "expiration_date": url.expiration_date
            }))
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Short URL not found"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Database error"
        })),
    }
}
