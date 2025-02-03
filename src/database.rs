use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{Error, Row, SqlitePool, migrate::Migrator};
use crate::models::Url;
static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[async_trait]
pub trait UrlRepository: Send + Sync {
    async fn init_db(&self) -> Result<(), Error>;
    async fn insert_url(&self, original_url: &str, short_url: &str) -> Result<i64, Error>;
    async fn get_url(&self, short_url: &str) -> Result<Option<Url>, Error>;
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
        MIGRATOR.run(&*self.pool).await?;
        Ok(())
    }

    async fn insert_url(&self, original_url: &str, short_url: &str) -> Result<i64, Error> {
        let rec = sqlx::query("INSERT INTO urls (original_url, short_url) VALUES (?, ?) RETURNING id")
            .bind(original_url)
            .bind(short_url)
            .fetch_one(&*self.pool)
            .await?;

        Ok(rec.get(0))
    }

    async fn get_url(&self, short_url: &str) -> Result<Option<Url>, Error> {
        let result = sqlx::query_as::<_, Url>(
            "SELECT id, original_url, short_url, created_at, expiration_date FROM urls WHERE short_url = ?",
        )
            .bind(short_url)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(result)
    }
}