use chrono::NaiveDateTime;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

pub static OTP_VALIDITY: u64 = 5 * 60; // 5 minutes

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Otp {
    pub otp_id: Uuid,
    pub otp: String,
    pub created_at: NaiveDateTime,
}

impl Otp {
    pub fn new() -> Self {
        Self {
            otp_id: Uuid::new_v4(),
            otp: Self::generate_otp().to_string(),
            created_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn generate_otp() -> String {
        let alphabet: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
        nanoid!(6, &alphabet)
    }

    pub async fn save(&self, pool: &PgPool, user_id: &Uuid) -> Result<Self, sqlx::Error> {
        let otp = sqlx::query_as::<_, Otp>(
            "INSERT INTO one_time_passwords (otp_id, otp, created_at) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(self.otp_id)
        .bind(&self.otp)
        .bind(self.created_at)
        .fetch_one(pool)
        .await;

        otp
    }
}
