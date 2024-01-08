use axum::{routing::get, Router};
use dotenv::dotenv;
use shuttle_runtime::CustomError;
use sqlx::postgres::PgPool;

mod database;
mod handlers;
mod pkg;
mod router;
pub mod validators;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    dotenv().ok();

    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let app_state = pkg::AppState { pool };
    let app = Router::new()
        .route("/", get(handlers::health_check))
        .nest("/v1/auth", router::auth_routes())
        .nest("/v1/pomodoro", router::pomodoro_routes())
        .nest("/v1/user", router::user_information_routes())
        .with_state(app_state);

    Ok(app.into())
}
