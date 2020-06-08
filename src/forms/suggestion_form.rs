
use validator::Validate;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Form {
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub emotion: String,
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub content: String,
    pub contact: String,
    pub account: String,
    pub user_avatar: String,
    pub user_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateForm {
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub emotion: String,
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub content: String,
    pub contact: String,
}