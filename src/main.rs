mod db;
mod error;
mod api;
mod handlers;

use error::*;
use handlers::*;

use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use sqlx::sqlite::SqlitePool;




#[tokio::main]
async fn main() -> LinkShortenerResult<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();

    let db_path = std::env::var("DATABASE_URL")?;
    // Initialize the SQLite database pool
    let db_pool = SqlitePool::connect(db_path.as_str()).await.unwrap();
    let url = format!("{}:{}", std::env::var("LOCAL_IP")?, std::env::var("LOCAL_PORT")?);

    // Create the `urls` table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS urls (
            id bigint,
            original_url TEXT NOT NULL,
            short_url TEXT NOT NULL
        )
    "#,
    )
        .execute(&db_pool)
        .await
        .unwrap();

    println!("Starting HTTP server");

    Ok(HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .service(web::resource("/shorten").route(web::post().to(shorten)))
            .service(web::resource("/{short_url}").route(web::get().to(redirect)))
    })
        .bind(url)?
        .run()
        .await?)

}
