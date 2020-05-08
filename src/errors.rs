use actix_web::{HttpResponse, ResponseError};
use crate::response::ApiResponse;
use derive_more::Display; 

#[derive(Debug, Display)]
pub enum ApiError {
    ValidatorError(String)
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::ValidatorError(error) => HttpResponse::Ok().json(ApiResponse::error(&error, ())),
        }
    }
}