use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{Error, Row, SqlitePool};
use crate::models::Url;
#[async_trait]
pub trait UrlRepository: Send + Sync {
    async fn init_db(&self) -> Result<(), Error>;
    async fn insert_url(&self, original_url: &str, short_url: &str) -> Result<i64, Error>;
    async fn get_url(&self, short_url: &str) -> Result<Url, Error>;
}
pub struct SqliteUrlRepository {
    pool: Arc<SqlitePool>
}


impl SqliteUrlRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl UrlRepository for SqliteUrlRepository {
    async fn init_db(&self) -> Result<(), Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS urls (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                original_url TEXT NOT NULL,
                short_url TEXT UNIQUE NOT NULL
            );"
        ).execute(&*self.pool).await?;
        Ok(())
    }

    async fn insert_url(&self, original_url: &str, short_url: &str) -> Result<i64, Error> {
        let result = sqlx::query(
            "INSERT INTO urls (original_url, short_url) VALUES (?, ?) RETURNING id"
        ).bind(original_url).bind(short_url).fetch_one(&*self.pool).await?;
        Ok(result.get::<i64, _>("id"))
    }

    async fn get_url(&self, short_url: &str) -> Result<Url, Error> {
        let result = sqlx::query_as::<_, Url>(
            "SELECT * FROM urls WHERE short_url = ?"
        ).bind(short_url).fetch_one(&*self.pool).await?;
        Ok(result)
    }
}