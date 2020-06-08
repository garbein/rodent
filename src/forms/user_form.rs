
use validator::Validate;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Form {
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub name: String,
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub email: String,
    pub avatar: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateForm {
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub name: String,
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub email: String,
    pub avatar: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUsernameForm {
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub name: String,
    pub id: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateAvatarForm {
    #[validate(length(min = 1, max = 255, message = "name length min 1 max 255"))]
    pub avatar: String,
    pub id: i32,
}