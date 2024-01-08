use crate::pkg::AppState;
use axum::extract::State;

pub struct AuthHandlers;

impl AuthHandlers {
    pub async fn sign_up(State(state): State<AppState>) {}
    pub async fn verify_email(State(state): State<AppState>) {}
    pub async fn request_new_verification_token(State(state): State<AppState>) {}
    pub async fn password_reset() {}
    pub async fn confirm_password_reset_token() {}
    pub async fn set_new_password() {}
    pub async fn login() {}
    pub async fn logout() {}
    pub async fn refresh_token() {}
}
