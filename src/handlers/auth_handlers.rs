use crate::database::models::UserInformation;
use crate::pkg::{AppState, SignupRequest};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use sqlx::query;

#[debug_handler(state = AppState)]
pub async fn sign_up(
    State(state): State<AppState>,
    Json(data): Json<SignupRequest>,
) -> impl IntoResponse{
    let new_user = UserInformation::new(
        &data.full_name,
        &data.password,
        &data.email,
        &data.occupation,
    )
    .await;
    let query = sqlx::query_as::<_, UserInformation>(
        "INSERT INTO user_information (id, full_name, email, occupation, password) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(new_user.id)
    .bind(new_user.full_name)
    .bind(new_user.email)
    .bind(new_user.occupation)
    .bind(new_user.password)
    .fetch_one(&state.pool)
    .await;

    match query {
        Ok(record) => Ok((StatusCode::CREATED, Json(record))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
    // (StatusCode::OK, Json(data))
}

pub async fn verify_email() {}
pub async fn request_new_verification_token() {}
pub async fn password_reset() {}
pub async fn confirm_password_reset_token() {}
pub async fn set_new_password() {}
pub async fn login() {}
pub async fn logout() {}
pub async fn refresh_token() {}
