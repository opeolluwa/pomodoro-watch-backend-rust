use axum::response::IntoResponse;

/// return the health status of the service
pub async fn health_check() -> impl IntoResponse {
    String::from("Service Active!")
}
