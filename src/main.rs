use axum::{
    routing::get,
    Router,
};
use dotenv::dotenv;
use shuttle_runtime::CustomError;
use sqlx::postgres::PgPool;
use tower_http::cors::{Any, CorsLayer};

mod database;
mod handlers;
mod pkg;
mod router;


#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    dotenv().ok();

    let cors = CorsLayer::new()
        // .allow_credentials(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let app_state = pkg::AppState { pool };
    let app = Router::new()
        .route("/", get(handlers::health_check))
        .nest("/v1/auth", router::auth_routes().await)
        .nest("/v1/pomodoro", router::pomodoro_routes().await)
        .nest("/v1/user", router::user_information_routes().await)
        .layer(cors)
        .with_state(app_state);

    Ok(app.into())
}
