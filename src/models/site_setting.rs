use crate::db::Pool;
use crate::forms::site_setting_form;
use chrono::prelude::*;
use sqlx::mysql::MySqlQueryAs;

pub async fn create(form: &site_setting_form::Form, pool: &Pool) -> u64 {
    let time = Local::now().timestamp();
    let rec = sqlx::query!(
        r#"insert into site_setting (name,title,content,status,ctime) values (?, ?, ?, ?, ?)"#,
        form.name,
        form.title,
        form.content,
        1,
        time
    )
    .execute(pool)
    .await;
    if rec.is_ok() {
        let last_id_rec: Result<(u64,), sqlx::error::Error> =
            sqlx::query_as("SELECT LAST_INSERT_ID()")
                .fetch_one(pool)
                .await;
        match last_id_rec {
            Ok(t) => t.0,
            _ => 0,
        }
    } else {
        0
    }
}

pub async fn update(name: &str, form: &site_setting_form::Form, pool: &Pool) -> bool {
    let time = Local::now().timestamp();
    let rec = sqlx::query!(
        r#"update site_setting set title = ?, content = ?, mtime = ? where name = ?"#,
        form.title,
        form.content,
        time,
        name
    )
    .execute(pool)
    .await;
    if rec.is_ok() {
        true
    }
    else {
        false
    }
}

pub async fn delete(name: &str, pool: &Pool) -> bool {
    let time = Local::now().timestamp();
    let rec = sqlx::query!(
        r#"update site_setting set deleted = 1, mtime = ? where name = ?"#,
        time,
        name
    )
    .execute(pool)
    .await;
    if rec.is_ok() {
        true
    }
    else {
        false
    }
}

pub async fn get_by_name(name: &str, pool: &Pool) -> String {
    let rec = sqlx::query!(r#"SELECT content from site_setting where name = ?"#, name)
        .fetch_one(pool)
        .await;
    match rec {
        Ok(row) => row.content,
        Err(_) => "".into(),
    }
}
