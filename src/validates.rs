use actix_web::web::Json;
use validator::{Validate, ValidationErrors};
use crate::errors::ApiError;

pub fn validate<T>(params: &Json<T>) -> Result<(), ApiError>
where
  T: Validate,
{
  match params.validate() {
    Ok(()) => Ok(()),
    Err(e) => Err(ApiError::ValidatorError(handle_errors(e).join(" "))),
  }
}

fn handle_errors(error: ValidationErrors) -> Vec<String> {
  error
    .field_errors()
    .into_iter()
    .map(|error| {
      let default_error = format!("{} is required", error.0);
      error.1[0]
        .message
        .as_ref()
        .unwrap_or(&std::borrow::Cow::Owned(default_error))
        .to_string()
    })
    .collect()
}