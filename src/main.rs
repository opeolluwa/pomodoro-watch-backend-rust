use axum::{routing::get, Router};
use dotenv::dotenv;
use shuttle_runtime::CustomError;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

mod pkg;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    dotenv().ok();

    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    //cors
    let _cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    // database
    let db_connection_str = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://opeolluwa:thunderstorm@localhost:5432/pomodoro".to_string()
    });

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let app_state = pkg::AppState { pool };
    let app = Router::new().route("/", get(|| async { "Hello, World!" }))
    // .layer(cors_layer)
    .with_state(app_state);

    Ok(app.into())
}
