use serde::{Deserialize, Serialize};

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

    pub fn err(message: &str) -> ApiResponse<()> {
        ApiResponse {
            message: message.to_string(),
            data: Some(()),
            success: false,
        }
    }
}
