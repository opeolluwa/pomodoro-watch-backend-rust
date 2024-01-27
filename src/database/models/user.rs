use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use bcrypt::{hash, DEFAULT_COST};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(sqlx::FromRow)]
pub struct UserInformation {
    pub id: Uuid,
    pub full_name: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String,
    pub occupation: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_verified: bool,
}

impl UserInformation {
    pub async fn new(full_name: &str, password: &str, email: &str, occupation: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            full_name: full_name.to_string().to_ascii_lowercase(),
            password: UserInformation::hash_password(password).await,
            email: email.to_string(),
            occupation: occupation.to_string().to_ascii_lowercase(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
            is_verified: false,
        }
    }

    async fn hash_password(password: &str) -> String {
        hash(password, DEFAULT_COST).unwrap()
    }
}

// A junction between the user and the otp table
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(sqlx::FromRow)]
pub struct UserAuth {
    pub id: Uuid,
    pub email: String,
    pub is_verified: bool,
    // pub user_id: Uuid,
    pub otp: String,
    pub created_at: NaiveDateTime,
}
