use crate::database::models::UserInformation;
use crate::pkg::{
    jwt::JwtClaims, ApiResponse, AppState, NewVerificationTokenRequest, SignupRequest,
    VerifyEmailRequest,
};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use serde_json::json;

#[debug_handler(state = AppState)]
pub async fn sign_up(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> impl IntoResponse {
    let new_user = UserInformation::new(
        &payload.full_name,
        &payload.password,
        &payload.email,
        &payload.occupation,
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
        Ok(data) => {
            // sign a new jwt
            let claim = JwtClaims::new(&data.email).gen_token();

            Ok((
                StatusCode::CREATED,
                Json(ApiResponse::new(
                    json!({"token":claim}),
                    "successfully created user accout",
                )),
            ))
        }
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn verify_email(
    State(state): State<AppState>,
    Json(payload): Json<VerifyEmailRequest>,
) -> impl IntoResponse {
}

pub async fn request_new_verification_token(
    State(state): State<AppState>,
    Json(payload): Json<NewVerificationTokenRequest>,
) -> impl IntoResponse {
}

pub async fn password_reset() {}
pub async fn confirm_password_reset_token() {}
pub async fn set_new_password() {}
pub async fn login() {}
pub async fn logout() {}
pub async fn refresh_token() {}
