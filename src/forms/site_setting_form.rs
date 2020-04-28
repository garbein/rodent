
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Form {
    pub name: String,
    pub title: String,
    pub content: String,
}