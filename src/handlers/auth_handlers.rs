use crate::database::models::{
    otp::Otp,
    user::{UserAuth, UserInformation},
};
use crate::pkg::api::{
    ApiResponse, NewVerificationTokenRequest, SignupRequest, VerifyEmailRequest,
};
use crate::pkg::email_templates::EmailTemplate;
use crate::pkg::jwt::JwtClaims;
use crate::pkg::mailer::Mailer;
use crate::pkg::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

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
             let otp = Otp::new().save(&state.pool).await.unwrap();
            // let Some(otp) = Otp::new().save(&state.pool).await.ok() else {
            //     return Err((
            //         StatusCode::BAD_REQUEST,
            //         "failed to generate otp".to_string(),
            //     ));
            // };
            let jwt_token = JwtClaims::new(&data.email).gen_token();
            Mailer::new(&data.email, EmailTemplate::VerifyEmail, Some(otp))
                .send_email()
                .await;

            Ok((
                StatusCode::CREATED,
                Json(ApiResponse::new(
                    json!({"token":jwt_token}),
                    "successfully created user accout",
                )),
            ))
        }
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn verify_email(
    State(state): State<AppState>,
    claim: JwtClaims,
    Json(payload): Json<VerifyEmailRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query =
        sqlx::query_as::<_, UserAuth>("SELECT one_time_passwords.otp, one_time_passwords.created_at FROM one_time_passwords INNER JOIN user_information ON one_time_passwords.id = user_information.otp_id")
            // .bind(&payload.otp)
            .bind(&claim.sub)
            .fetch_one(&state.pool)
            .await;

    match query {
        Ok(data) => {
            // if data.is_verified {
            //     return Err((
            //         StatusCode::BAD_REQUEST,
            //         "user account already verified".to_string(),
            //     ));
            // }

            println!("{:#?}", data);

     

            Ok((
                StatusCode::CREATED,
                Json(ApiResponse::new(
                    json!({"message":"successfully sent verification email"}),
                    "successfully sent verification email",
                )),
            ))
        }
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn request_new_verification_token(
    State(_state): State<AppState>,
    Json(_payload): Json<NewVerificationTokenRequest>,
) -> impl IntoResponse {
}

pub async fn password_reset() {}
pub async fn confirm_password_reset_token() {}
pub async fn set_new_password() {}
pub async fn login() {}
pub async fn logout() {}
pub async fn refresh_token() {}
