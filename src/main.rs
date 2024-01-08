use axum::{routing::get, Router};
use dotenv::dotenv;
use shuttle_runtime::CustomError;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

mod database;
mod handlers;
mod pkg;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    dotenv().ok();

    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let app_state = pkg::AppState { pool };
    let app = Router::new()
        .route("/", get(|| async { "Service active!" }))
        .with_state(app_state);

    Ok(app.into())
}
