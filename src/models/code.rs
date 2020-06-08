use serde::{Serialize, Deserialize};

use crate::utils::email;
use crate::utils::sms;

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub enum SendType {
    Email, 
    Sms
}

pub async fn send_by_email(account: &str, content: &str) -> anyhow::Result<()> {
    email::send_mail_simply(account, content);
    Ok(())
}

pub async fn send_by_sms(account: &str, content: &str) -> anyhow::Result<()> {
    sms::send_sms(account, content).await?;
    Ok(())
}
