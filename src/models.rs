use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio::sync::RwLock;
use crate::database::UrlRepository;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Url {
    id: i64,
    pub original_url: String,
    pub short_url: String,
    pub created_at: String,
    pub expiration_date: String,
}

#[derive(Deserialize, Debug)]
pub struct ShortenRequest {
    pub original_url: String,
    pub custom_alias: Option<String>,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
    pub expiration_date: String,
}

pub type DB = Arc<RwLock<dyn UrlRepository + Send + Sync>>;