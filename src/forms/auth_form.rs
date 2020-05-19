use validator::Validate;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct RegisterForm {
    #[validate(length(min = 3, max = 32, message = "name length min 3 max 32"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 5, max = 16, message = "password length min 5 max 16"))]
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginForm {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 5, max = 16, message = "name length min 5 max 16"))]
    pub password: String,
}
