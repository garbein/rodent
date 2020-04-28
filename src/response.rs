use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i16,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {

    pub fn error(message: &str, data: T) -> ApiResponse<T> {
        ApiResponse {
            code: 0,
            message: message.into(),
            data,
        }
    }

    pub fn succues(data: T, message: &str) -> ApiResponse<T> {
        ApiResponse {
            code: 1,
            message: message.into(),
            data,
        }
    }
}
