
use validator::Validate;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Form {
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub name: String,
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateForm {
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub title: String,
    pub content: String,
}