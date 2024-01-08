use axum::response::IntoResponse;

pub mod auth_handlers;
pub(crate) mod pomodoro;
pub mod user;

/// return the health status of the service
pub async fn health_check() -> impl IntoResponse {
    String::from("Service Active!")
}
