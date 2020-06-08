use crate::db::Pool;
use crate::forms::suggestion_form;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "sqlite")]
use sqlx::sqlite::SqliteQueryAs;
#[cfg(feature = "mysql")]
use sqlx::mysql::MySqlQueryAs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Suggestion {
    pub id: i32,
    pub account: String,
    pub user_avatar: String,
    pub user_name: String,
    pub emotion: String,
    pub contact: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn get_all(pool: &Pool, page: i32, size: i32) -> anyhow::Result<(i64, Vec<Suggestion>)> {
    let (total,):(i64,) = sqlx::query_as("select count(*) as total from suggestion where deleted = 0")
        .fetch_one(pool)
        .await?;
    let recs = sqlx::query!(
        r#"select * from suggestion where deleted = 0 limit ? offset ?"#,
        size,
        (page - 1) * size
    )
    .fetch_all(pool)
    .await?;
    let mut list: Vec<Suggestion> = Vec::new();
    for rec in recs {
        let s = Suggestion {
            id: rec.id,
            account: rec.account,
            user_avatar: rec.user_avatar,
            user_name: rec.user_name,
            emotion: rec.emotion,
            content: rec.content,
            contact: rec.contact,
            created_at: if rec.created_at == 0 {
                "".into()
            } else {
                Local
                    .timestamp(rec.created_at as i64, 0)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            },
            updated_at: if rec.updated_at == 0 {
                "".into()
            } else {
                Local
                    .timestamp(rec.updated_at as i64, 0)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            },
        };
        list.push(s);
    }
    println!("{:?}", list);
    Ok((total as i64, list))
}

pub async fn create(form: &suggestion_form::Form, pool: &Pool) -> anyhow::Result<i64> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"insert into suggestion (account,user_avatar,user_name,emotion,content,contact,status,created_at) values (?, ?, ?, ?, ?, ?, ?, ?)"#,
        form.account,
        form.user_avatar,
        form.user_name,
        form.emotion,
        form.content,
        form.contact,
        1,
        time
    )
    .execute(pool)
    .await?;
    let last_id_rec: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(pool)
        .await?;
    Ok(last_id_rec.0 as i64)
}

pub async fn update(
    id: i64,
    form: &suggestion_form::UpdateForm,
    pool: &Pool,
) -> anyhow::Result<bool> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"update suggestion set emotion = ?, content = ?, contact = ?, updated_at = ? where id = ?"#,
        form.emotion,
        form.content,
        form.contact,
        time,
        id
    )
    .execute(pool)
    .await?;
    Ok(true)
}

pub async fn delete(id: i64, pool: &Pool) -> anyhow::Result<bool> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"update suggestion set deleted = 1, updated_at = ? where id = ?"#,
        time,
        id
    )
    .execute(pool)
    .await?;
    Ok(true)
}

pub async fn get_by_id(id: i64, pool: &Pool) -> anyhow::Result<Suggestion> {
    let rec = sqlx::query!(r#"SELECT * from suggestion where id = ?"#, id as i64)
        .fetch_one(pool)
        .await?;
    let s = Suggestion {
        id: rec.id,
        account: rec.account,
        user_avatar: rec.user_avatar,
        user_name: rec.user_name,
        emotion: rec.emotion,
        content: rec.content,
        contact: rec.contact,
        created_at: if rec.created_at == 0 {
            "".into()
        } else {
            Local
                .timestamp(rec.created_at as i64, 0)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        },
        updated_at: if rec.updated_at == 0 {
            "".into()
        } else {
            Local
                .timestamp(rec.updated_at as i64, 0)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        },
    };
    Ok(s)
}

pub async fn get_content_by_id(id: i64, pool: &Pool) -> anyhow::Result<String> {
    let rec = sqlx::query!(r#"SELECT content from suggestion where id = ?"#, id)
        .fetch_one(pool)
        .await?;
    Ok(rec.content)
}