use std::time::SystemTime;

use axum::extract;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    // note that in production, you will probably want to use a random SHA-256 hash or similar
    let secret = "JWT_SECRET".to_string();
    Keys::new(secret.as_bytes())
});

// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    sub: String,
    exp: usize,
}

impl JwtClaims {
    pub fn new(sub: &str) -> Self {
        // add 10 minutes to current unix epoch time as expiry date/time
        let exp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + (10 * 60 as u64); // 10 minutes

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
