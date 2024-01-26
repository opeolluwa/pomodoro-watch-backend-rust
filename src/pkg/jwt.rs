use std::time::SystemTime;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    // note that in production, you will probably want to use a random SHA-256 hash or similar
    let secret = "JWT_SECRET".to_string();
    Keys::new(secret.as_bytes())
});

// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
   pub sub: String,
    exp: usize,
}

impl JwtClaims {
    pub fn new(sub: &str) -> Self {
        // add 10 minutes to current unix epoch time as expiry date/time
        let exp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + (10 * 60_u64); // 10 minutes

        Self {
            sub: sub.to_string(),
            exp: exp as usize,
        }
    }

    pub fn gen_token(&self) -> String {
        encode(&Header::default(), self, &KEYS.encoding).unwrap()
    }

    pub fn extract_token(token: &str) -> Result<Self, String> {
        decode::<Self>(token, &KEYS.decoding, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| e.to_string())
    }
}

// encoding/decoding keys - set in the static `once_cell` above
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
// error types for auth errors
#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}


// implement IntoResponse for AuthError so we can use it as an Axum response type
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}


// implement FromRequestParts for Claims (the JWT struct)
// FromRequestParts allows us to use Claims without consuming the request
#[async_trait]
impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<JwtClaims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
