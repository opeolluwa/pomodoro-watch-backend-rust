use crate::database::models::{
    otp::{Otp, OTP_VALIDITY},
    user::{UserAuth, UserInformation},
};
use crate::pkg::http_response::ApiResponse;

use crate::pkg::email_templates::EmailTemplate;
use crate::pkg::http_request::auth as http_request;
use crate::pkg::jwt::JwtClaims;
use crate::pkg::mailer::Mailer;
use crate::pkg::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};

pub async fn sign_up(
    State(state): State<AppState>,
    Json(payload): Json<http_request::Signup>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
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
            let Some(otp) = Otp::new().save(&state.pool).await.ok() else {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<UserInformation>::err(
                        "failed to generate verification otp",
                    )),
                ));
            };

            // update the user_information for which the token was generated
            let _ = sqlx::query_as::<_, UserInformation>(
                "UPDATE user_information SET otp_id = $1 WHERE id = $2",
            )
            .bind(otp.otp_id)
            .bind(data.id)
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
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<UserInformation>::err(&err.to_string())),
        )),
    }
}

pub async fn verify_email(
    State(state): State<AppState>,
    claim: JwtClaims,
    Json(payload): Json<http_request::VerifyEmail>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
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
            .bind(data.id)
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
    State(state): State<AppState>,
    Json(payload): Json<http_request::NewVerificationToken>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    let query = sqlx::query_as::<_, UserInformation>("SELECT * FROM user_information")
        .bind(&payload.email)
        .fetch_one(&state.pool)
        .await;

    match query {
        Ok(data) => {
            // let otp = Otp::new().save(&state.pool, &data.id).await.unwrap();
            let Some(otp) = Otp::new().save(&state.pool).await.ok() else {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new((), "failed to generate verification otp")),
                ));
            };

            // update the user_information for which the token was generated
            let _ = sqlx::query_as::<_, UserInformation>(
                "UPDATE user_information SET otp_id = $1 WHERE id = $2",
            )
            .bind(otp.otp_id)
            .bind(data.id)
            .fetch_one(&state.pool)
            .await;

            let jwt_token = JwtClaims::new(&data.email).gen_token();
            Mailer::new(&data.email, EmailTemplate::VerifyEmail, Some(otp))
                .send_email()
                .await;

            Ok((
                StatusCode::OK,
                Json(ApiResponse::new(
                    json!({"token":jwt_token}),
                    "successfully created user accout",
                )),
            ))
        }

        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<UserInformation>::err(&err.to_string())),
        )),
    }
}

pub async fn password_reset(
    State(state): State<AppState>,
    Json(payload): Json<http_request::NewVerificationToken>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    let query = sqlx::query_as::<_, UserInformation>("SELECT * FROM user_information")
        .bind(&payload.email)
        .fetch_one(&state.pool)
        .await;

    match query {
        Ok(data) => {
            let Some(otp) = Otp::new().save(&state.pool).await.ok() else {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new((), "failed to generate verification otp")),
                ));
            };

            // update the user_information for which the token was generated
            let _ = sqlx::query_as::<_, UserInformation>(
                "UPDATE user_information SET otp_id = $1 WHERE id = $2",
            )
            .bind(otp.otp_id)
            .bind(data.id)
            .fetch_one(&state.pool)
            .await;

            let jwt_token = JwtClaims::new(&data.email).gen_token();
            Mailer::new(&data.email, EmailTemplate::VerifyEmail, Some(otp))
                .send_email()
                .await;

            Ok((
                StatusCode::OK,
                Json(ApiResponse::new(
                    json!({"token":jwt_token}),
                    "successfully generated password reset token, see your email for further instructions",
                )),
            ))
        }

        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<UserInformation>::err(&err.to_string())),
        )),
    }
}

pub async fn confirm_password_reset_token(
    claims: JwtClaims,
    State(state): State<AppState>,
    Json(payload): Json<http_request::ConfirmPasswordReset>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    let query = sqlx::query_as::<_, UserAuth>("SELECT * FROM user_information FULL JOIN one_time_passwords ON user_information.otp_id = one_time_passwords.otp_id WHERE email = $1").bind(&claims.sub).fetch_one(&state.pool).await;

    match query {
        Ok(data) => {
            if payload.otp != data.otp
                || data.created_at + chrono::Duration::minutes(OTP_VALIDITY.try_into().unwrap())
                    < chrono::Local::now().naive_local()
            {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<UserInformation>::err(
                        "invalid otp or otp expired",
                    )),
                ));
            }

            let jwt_token = JwtClaims::new(&claims.sub).gen_token();

            Ok((
                StatusCode::OK,
                Json(ApiResponse::new(
                    json!({"token":jwt_token}),
                    "OTP successfully verifried",
                )),
            ))
        }

        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::err(&e.to_string())),
            ))
        }
    }
}
pub async fn set_new_password(
    claims: JwtClaims,
    State(state): State<AppState>,
    Json(payload): Json<http_request::NewPassword>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> {
    if payload.new_password.trim() != payload.confirm_password.trim() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<UserInformation>::err(
                "passwords do not match",
            )),
        ));
    }

    let Some(_) = UserInformation::fetch(&claims.sub, &state.pool).await.ok() else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<UserInformation>::err(
                "passwords do not match",
            )),
        ));
    };

    let Some(_) =
        UserInformation::update_password(&claims.sub, payload.new_password.trim(), &state.pool)
            .await
            .ok()
    else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<UserInformation>::err(
                "error whhile updating password, please retry after some time",
            )),
        ));
    };

    Ok((
        StatusCode::OK,
        Json(ApiResponse::new((), "password updated successfully")),
    ))
}
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<http_request::Login>,
) -> Result<(StatusCode, Json<ApiResponse<Value>>), (StatusCode, Json<ApiResponse<()>>)> {
    let Some(user) = UserInformation::fetch(&payload.email, &state.pool)
        .await
        .ok()
    else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<UserInformation>::err(
                "invalid username or password",
            )),
        ));
    };

    let is_correct_password =
        UserInformation::compare_password(payload.password.trim(), &user.password).await;

    if !is_correct_password {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::<UserInformation>::err(
                "invalid email or password",
            )),
        ));
    }

    let jwt_token = JwtClaims::new(&user.email).gen_token();
    Ok((
        StatusCode::OK,
        Json(ApiResponse::new(
            json!({"jwt":jwt_token}),
            "user successfully logged in",
        )),
    ))
}
pub async fn logout(_claims: JwtClaims, State(
    _state): State<AppState>) {}

pub async fn refresh_token(_claims: JwtClaims, State(_state): State<AppState>) {}
