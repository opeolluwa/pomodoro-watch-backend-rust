use chrono::NaiveDateTime;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Otp {
    pub id: Uuid,
    pub otp: String,
    pub created_at: NaiveDateTime,
}

impl Otp {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            otp: Self::generate_otp().to_string(),
            created_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn generate_otp() -> String {
        let alphabet: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
        nanoid!(6, &alphabet)
    }

    pub async fn save(&self, pool: &PgPool) -> Result<Self, sqlx::Error> {
        let query = sqlx::query_as::<_, Otp>(
            "INSERT INTO one_time_passwords (id, otp, created_at) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(self.id)
        .bind(&self.otp)
        .bind(self.created_at)
        .fetch_one(pool)
        .await;

        query
    }
}
