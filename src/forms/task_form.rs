
use validator::Validate;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Form {
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub name: String,
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub detail_list: String,
    pub icon_bean: String,
    pub r#type: i32,
    pub detail_num: i32,
    pub change_times: i32,
    pub progress: i32,
    pub start_at: String,
    pub end_at: String,
    pub finished_at: String,
    pub remark: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateForm {
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub name: String,
    #[validate(length(min = 1, max = 32, message = "name length min 1 max 32"))]
    pub detail_list: String,
    pub icon_bean: String,
    pub r#type: i32,
    pub detail_num: i32,
    pub change_times: i32,
    pub progress: i32,
    pub start_at: String,
    pub end_at: String,
    pub finished_at: String,
    pub remark: String,
}