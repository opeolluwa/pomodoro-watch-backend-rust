use crate::database::models::{
    otp::{Otp, OTP_VALIDITY},
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
            // let otp = Otp::new().save(&state.pool, &data.id).await.unwrap();
            let Some(otp) = Otp::new().save(&state.pool, &data.id).await.ok() else {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to generate verification otp".to_string(),
                ));
            };

            // update the user_information for which the token was generated
            let _ = sqlx::query_as::<_, UserInformation>(
                "UPDATE user_information SET otp_id = $1 WHERE id = $2",
            )
            .bind(&otp.otp_id)
            .bind(&data.id)
            .fetch_one(&state.pool)
            .await;

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
    println!("hey {:#?}", &claim.sub);
    let query =
        sqlx::query_as::<_, UserAuth>("SELECT * FROM user_information FULL JOIN one_time_passwords ON user_information.otp_id = one_time_passwords.otp_id WHERE email = $1")
            .bind(&claim.sub)
            .fetch_one(&state.pool)
            .await;

    match query {
        Ok(data) => {
            if data.is_verified {
                return Err((
                    StatusCode::CONFLICT,
                    "user account already verified".to_string(),
                ));
            }

            // confirm the otp sent to the user and the validity time
            if payload.otp != data.otp
                || data.created_at + chrono::Duration::minutes(OTP_VALIDITY.try_into().unwrap())
                    < chrono::Local::now().naive_local()
            {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "invalid otp or otp expired".to_string(),
                ));
            }

            // set the user account to verified
            let _ = sqlx::query_as::<_, UserAuth>(
                "UPDATE user_information SET is_verified = $1 WHERE id = $2",
            )
            .bind(true)
            .bind(&data.id)
            .fetch_one(&state.pool)
            .await;

            Ok((
                StatusCode::CREATED,
                Json(ApiResponse::new(
                    None::<UserAuth>,
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
