use std::sync::Arc;
use actix_web::{App, HttpServer};
use actix_web::middleware::DefaultHeaders;
use actix_web::web;
use sqlx::SqlitePool;
use crate::config::init;
use crate::database::{SqliteUrlRepository, UrlRepository};

mod database;
mod config;
mod models;
mod handlers;
mod routes;
mod hash;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init();

    let db_path = dotenv::var("DATABASE_URL").expect("There's no DATABASE_URL variable in .env file");
    let pool = SqlitePool::connect(&db_path).await.unwrap();

    let db_repo: Arc<dyn UrlRepository + Send + Sync> = Arc::new(SqliteUrlRepository::new(Arc::new(pool)));

    db_repo.init_db().await.expect("Initialization of database is aborted");

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
        .await
}
