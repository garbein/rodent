use crate::db::Pool;
use crate::forms::site_setting_form;
use chrono::prelude::*;
use sqlx::mysql::MySqlQueryAs;

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
    let last_id_rec: (u64, ) = sqlx::query_as("SELECT LAST_INSERT_ID()")
        .fetch_one(pool)
        .await?;
    Ok(last_id_rec.0)
}

pub async fn update(name: &str, form: &site_setting_form::UpdateForm, pool: &Pool) -> anyhow::Result<bool> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"update site_setting set title = ?, content = ?, mtime = ? where name = ?"#,
        form.title,
        form.content,
        time,
        name
    )
    .execute(pool)
    .await?;
    Ok(true)
}

pub async fn delete(name: &str, pool: &Pool) -> anyhow::Result<bool> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"update site_setting set deleted = 1, mtime = ? where name = ?"#,
        time,
        name
    )
    .execute(pool)
    .await?;
    Ok(true)
}

pub async fn get_by_name(name: &str, pool: &Pool) -> anyhow::Result<String> {
    let rec = sqlx::query!(r#"SELECT content from site_setting where name = ?"#, name)
        .fetch_one(pool)
        .await?;
    Ok(rec.content)
}
