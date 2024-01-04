use axum::{routing::get, Router};
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() {
    //tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    //cors
    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    // database
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // env
    dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or("8009".to_string())
        .parse::<u16>()
        .unwrap_or(8009);
    let app_address = SocketAddr::from(([0, 0, 0, 0], port));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(cors_layer)
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(app_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
