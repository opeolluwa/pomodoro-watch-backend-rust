pub mod jwt;
pub mod mailer;

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
    pub full_name: String,
    pub password: String,
    pub email: String,
    pub occupation: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyEmailRequest {
    pub otp: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewVerificationTokenRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgottenPasswordRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtpRequest {
    pub otp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: Option<T>,
    pub success: bool,
}

impl<T: Send + Clone> ApiResponse<T> {
    pub fn new(data: T, message: &str) -> ApiResponse<T> {
        ApiResponse {
            message: message.to_string(),
            data: Some(data),
            success: true,
        }
    }
}
