/// 配置模型
use crate::db::Pool;
use crate::forms::site_setting_form;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "sqlite")]
use sqlx::sqlite::SqliteQueryAs;
#[cfg(feature = "mysql")]
use sqlx::mysql::MySqlQueryAs;

/// 配置结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 获取列表
pub async fn get_all(pool: &Pool, page: i32, size: i32) -> anyhow::Result<(i64, Vec<Setting>)> {
    let (total,):(i64,) = sqlx::query_as("select count(*) as total from site_setting where deleted = 0")
        .fetch_one(pool)
        .await?;
    let recs = sqlx::query!(
        r#"select * from site_setting where deleted = 0 limit ? offset ?"#,
        size,
        (page - 1) * size
    )
    .fetch_all(pool)
    .await?;
    let mut list: Vec<Setting> = Vec::new();
    for rec in recs {
        let s = Setting {
            id: rec.id,
            name: rec.name,
            title: rec.title,
            content: rec.content,
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
    Ok((total as i64, list))
}

/// 创建
pub async fn create(form: &site_setting_form::Form, pool: &Pool) -> anyhow::Result<i64> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"insert into site_setting (name,title,content,status,created_at) values (?, ?, ?, ?, ?)"#,
        form.name,
        form.title,
        form.content,
        1,
        time
    )
    .execute(pool)
    .await?;

    #[cfg(feature = "sqlite")]
    type LastInsertType = i64;
    #[cfg(feature = "sqlite")]
    let sql = "SELECT last_insert_rowid()";
    #[cfg(feature = "mysql")]
    type LastInsertType = u64;
    #[cfg(feature = "mysql")]
    let sql = "SELECT last_insert_id()";

    let last_id_rec: (LastInsertType,) = sqlx::query_as(sql)
        .fetch_one(pool)
        .await?;
    Ok(last_id_rec.0 as i64)
}

/// 更新
pub async fn update(
    id: i64,
    form: &site_setting_form::UpdateForm,
    pool: &Pool,
) -> anyhow::Result<bool> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"update site_setting set name = ?, title = ?, content = ?, updated_at = ? where id = ?"#,
        form.name,
        form.title,
        form.content,
        time,
        id
    )
    .execute(pool)
    .await?;
    Ok(true)
}

/// 删除
pub async fn delete(id: i64, pool: &Pool) -> anyhow::Result<bool> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"update site_setting set deleted = 1, updated_at = ? where id = ?"#,
        time,
        id
    )
    .execute(pool)
    .await?;
    Ok(true)
}

/// 通过id获取配置
pub async fn get_by_id(id: i64, pool: &Pool) -> anyhow::Result<Setting> {
    let rec = sqlx::query!(r#"SELECT * from site_setting where id = ?"#, id as i64)
        .fetch_one(pool)
        .await?;
    let s = Setting {
        id: rec.id,
        name: rec.name,
        title: rec.title,
        content: rec.content,
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

/// 通过id获取配置内容
pub async fn get_content_by_id(id: i64, pool: &Pool) -> anyhow::Result<String> {
    let rec = sqlx::query!(r#"SELECT content from site_setting where id = ?"#, id)
        .fetch_one(pool)
        .await?;
    Ok(rec.content)
}