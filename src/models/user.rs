use crate::db::Pool;
use crate::forms::auth_form::RegisterForm;
use chrono::prelude::*;
use sqlx::mysql::MySqlQueryAs;

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn get_by_email(pool: &Pool, email: &str) -> anyhow::Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, name, email, password from user where email = ? and deleted = 0"#,
        email
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn create(pool: &Pool, form: &RegisterForm) -> anyhow::Result<u32> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"insert into user (name, email, password, ctime) values (?, ?, ?, ?)"#,
        form.name,
        form.email,
        form.password,
        time
    )
    .execute(pool)
    .await?;
    let last_id_rec: (u32, ) = sqlx::query_as("SELECT LAST_INSERT_ID()")
        .fetch_one(pool)
        .await?;
    Ok(last_id_rec.0)
}
