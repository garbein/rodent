use crate::db::Pool;
use crate::forms::site_setting_form;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlQueryAs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub id: u32,
    pub name: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn get_all(pool: &Pool, page: u32, size: u32) -> anyhow::Result<(u64, Vec<Setting>)> {
    let total_rec = sqlx::query!("select count(*) as total from site_setting where deleted = 0")
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
            created_at: if rec.ctime == 0 {
                "".into()
            } else {
                Local
                    .timestamp(rec.ctime as i64, 0)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            },
            updated_at: if rec.mtime == 0 {
                "".into()
            } else {
                Local
                    .timestamp(rec.mtime as i64, 0)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            },
        };
        list.push(s);
    }
    println!("{:?}", list);
    Ok((total_rec.total as u64, list))
}

pub async fn create(form: &site_setting_form::Form, pool: &Pool) -> anyhow::Result<u64> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"insert into site_setting (name,title,content,status,ctime) values (?, ?, ?, ?, ?)"#,
        form.name,
        form.title,
        form.content,
        1,
        time
    )
    .execute(pool)
    .await?;
    let last_id_rec: (u64,) = sqlx::query_as("SELECT LAST_INSERT_ID()")
        .fetch_one(pool)
        .await?;
    Ok(last_id_rec.0)
}

pub async fn update(
    id: u64,
    form: &site_setting_form::UpdateForm,
    pool: &Pool,
) -> anyhow::Result<bool> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"update site_setting set name = ?, title = ?, content = ?, mtime = ? where id = ?"#,
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

pub async fn delete(id: u64, pool: &Pool) -> anyhow::Result<bool> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"update site_setting set deleted = 1, mtime = ? where id = ?"#,
        time,
        id
    )
    .execute(pool)
    .await?;
    Ok(true)
}

pub async fn get_by_name(id: u64, pool: &Pool) -> anyhow::Result<String> {
    let rec = sqlx::query!(r#"SELECT content from site_setting where name = ?"#, id)
        .fetch_one(pool)
        .await?;
    Ok(rec.content)
}
