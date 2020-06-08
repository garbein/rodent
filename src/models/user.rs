use crate::db::Pool;
use crate::forms::auth_form::RegisterForm;
use crate::forms::user_form::UpdateUsernameForm;
use crate::forms::user_form::UpdateAvatarForm;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[cfg(feature = "sqlite")]
use sqlx::sqlite::SqliteQueryAs;
#[cfg(feature = "mysql")]
use sqlx::mysql::MySqlQueryAs;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn get_by_id(pool: &Pool, id: i32) -> anyhow::Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, name, email, password from user where id = ? and deleted = 0"#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
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

pub async fn create(pool: &Pool, form: &RegisterForm) -> anyhow::Result<i32> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"insert into user (name, email, password, created_at) values (?, ?, ?, ?)"#,
        form.name,
        form.email,
        form.password,
        time
    )
    .execute(pool)
    .await?;
    let last_id_rec: (i32, ) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(pool)
        .await?;
    Ok(last_id_rec.0)
}

pub async fn update_username(
    pool: &Pool,
    form: &UpdateUsernameForm,
) -> anyhow::Result<bool> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"update user set name = ?, updated_at = ? where id = ?"#,
        form.name,
        time,
        form.id
    )
    .execute(pool)
    .await?;
    Ok(true)
}

pub async fn update_avatar(
    pool: &Pool,
    form: &UpdateAvatarForm,
) -> anyhow::Result<bool> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"update user set avatar = ?, updated_at = ? where id = ?"#,
        form.avatar,
        time,
        form.id
    )
    .execute(pool)
    .await?;
    Ok(true)
}