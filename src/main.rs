use std::sync::Arc;
use actix_web::{App, HttpServer};
use actix_web::web;
use sqlx::SqlitePool;
use crate::config::init;
use crate::database::{SqliteUrlRepository, UrlRepository};

mod database;
mod config;
mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init();

    let db_path = dotenv::var("DATABASE_URL").expect("There's no DB_PATH variable in .env file");
    let pool = SqlitePool::connect(&db_path).await.unwrap();

    let db_repo: Arc<dyn UrlRepository + Send + Sync> = Arc::new(SqliteUrlRepository::new(Arc::new(pool)));

    db_repo.init_db().await.expect("Initialization of database is aborted");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_repo.clone()))
            .configure(config::actix_config)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
