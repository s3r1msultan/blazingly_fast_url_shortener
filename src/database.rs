use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{Error, Row, SqlitePool, migrate::Migrator};
use tokio::{sync::RwLock, time::{self, Duration}};
use chrono::Utc;
use crate::models::Url;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[async_trait]
pub trait UrlRepository: Send + Sync {
    async fn init_db(&self) -> Result<(), Error>;
    async fn insert_url(&self, original_url: &str, short_url: &str) -> Result<i64, Error>;
    async fn get_url(&self, short_url: &str) -> Result<Option<Url>, Error>;
    async fn delete_expired_urls(&self) -> Result<(), Error>;
}

pub struct SqliteUrlRepository {
    pool: Arc<RwLock<SqlitePool>>, // Use `RwLock` for mutable access
}

impl SqliteUrlRepository {
    pub fn new(pool: Arc<RwLock<SqlitePool>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UrlRepository for SqliteUrlRepository {
    async fn init_db(&self) -> Result<(), Error> {
        MIGRATOR.run(&*self.pool.read().await).await?;
        Ok(())
    }

    async fn insert_url(&self, original_url: &str, short_url: &str) -> Result<i64, Error> {
        let rec = sqlx::query("INSERT INTO urls (original_url, short_url) VALUES (?, ?) RETURNING id")
            .bind(original_url)
            .bind(short_url)
            .fetch_one(&*self.pool.read().await)
            .await?;

        Ok(rec.get(0))
    }

    async fn get_url(&self, short_url: &str) -> Result<Option<Url>, Error> {
        let result = sqlx::query_as::<_, Url>(
            "SELECT id, original_url, short_url, created_at, expiration_date FROM urls WHERE short_url = ?",
        )
            .bind(short_url)
            .fetch_optional(&*self.pool.read().await)
            .await?;

        Ok(result)
    }

    async fn delete_expired_urls(&self) -> Result<(), Error> {
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let result = sqlx::query("DELETE FROM urls WHERE expiration_date <= ?")
            .bind(now)
            .execute(&*self.pool.write().await) // Acquire write lock
            .await?;

        println!("Deleted {} expired URLs", result.rows_affected());
        Ok(())
    }
}


pub async fn ttl_cleanup_task(db: Arc<RwLock<dyn UrlRepository + Send + Sync>>) {
    let mut interval = time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        match db.write().await.delete_expired_urls().await {
            Ok(_) => println!("Expired URLs cleaned up."),
            Err(e) => eprintln!("Failed to clean expired URLs: {}", e),
        }
    }
}
