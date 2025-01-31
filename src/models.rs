use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Url {
    id: i64,
    pub original_url: String,
    pub short_url: String,
}

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub original_url: String,
    pub custom_alias: Option<String>,
}