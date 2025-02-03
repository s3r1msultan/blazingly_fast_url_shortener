use crate::database::ttl_cleanup_task;
use std::sync::Arc;
use actix_web::{App, HttpServer};
use actix_web::middleware::DefaultHeaders;
use actix_web::web;
use sqlx::SqlitePool;
use tokio::sync::RwLock;
use crate::config::init;
use crate::database::{SqliteUrlRepository, UrlRepository};
use crate::models::DB;

mod database;
mod config;
mod models;
mod handlers;
mod routes;
mod hash;
mod validation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init();

    let database_url = dotenv::var("DATABASE_URL").expect("There's no DATABASE_URL variable in .env file");
    let pool = SqlitePool::connect(&database_url).await.expect("Failed to connect to database");

    let db_repo: DB = Arc::new(RwLock::new(SqliteUrlRepository::new(Arc::new(RwLock::new(pool)))));

    db_repo.read().await.init_db().await.expect("Initialization of database is aborted");

    let cleanup_task = tokio::spawn(ttl_cleanup_task(db_repo.clone()));

    HttpServer::new(move || {

        App::new()
            .app_data(web::Data::new(db_repo.clone()))
            .wrap(DefaultHeaders::new()
                      .add(("X-Content-Type-Options", "nosniff"))
                      .add(("X-Frame-Options", "DENY"))
                      .add(("X-XSS-Protection", "1; mode=block")))
            .configure(routes::routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    cleanup_task.await?;
    Ok(())
}
