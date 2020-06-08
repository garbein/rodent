use crate::db::Pool;
use crate::forms::task_form;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "sqlite")]
use sqlx::sqlite::SqliteQueryAs;
#[cfg(feature = "mysql")]
use sqlx::mysql::MySqlQueryAs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub r#type: i32,
    pub remark: String,
    pub icon_bean: String,
    pub detail_list: String,
    pub detail_num: i32,
    pub change_times: i32,
    pub progress: i32,
    pub start_at: String,
    pub end_at: String,
    pub finished_at: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn get_all(pool: &Pool, page: i32, size: i32) -> anyhow::Result<(i64, Vec<Task>)> {
    let (total,):(i64,) = sqlx::query_as("select count(*) as total from task where deleted = 0")
        .fetch_one(pool)
        .await?;
    let recs = sqlx::query!(
        r#"select * from task where deleted = 0 limit ? offset ?"#,
        size,
        (page - 1) * size
    )
    .fetch_all(pool)
    .await?;
    let mut list: Vec<Task> = Vec::new();
    for rec in recs {
        let s = Task {
            id: rec.id,
            name: rec.name,
            r#type: rec.r#type as i32,
            icon_bean: rec.icon_bean,
            detail_list: rec.detail_list,
            detail_num: rec.detail_num,
            change_times: rec.change_times,
            progress: rec.progress as i32,
            remark: rec.remark,
            start_at: if rec.start_at == 0 {
                "".into()
            } else {
                Local
                    .timestamp(rec.start_at as i64, 0)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            },
            end_at: if rec.end_at == 0 {
                "".into()
            } else {
                Local
                    .timestamp(rec.end_at as i64, 0)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            },
            finished_at: if rec.finished_at == 0 {
                "".into()
            } else {
                Local
                    .timestamp(rec.finished_at as i64, 0)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            },
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

pub async fn create(form: &task_form::Form, pool: &Pool) -> anyhow::Result<i64> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"insert into task (name,type,icon_bean,detail_list,detail_num,progress,status,remark,created_at,start_at,end_at) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        form.name,
        form.r#type,
        form.icon_bean,
        form.detail_list,
        form.detail_num,
        form.progress,
        1,
        "",
        time,
        form.start_at,
        form.end_at
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
    form: &task_form::UpdateForm,
    pool: &Pool,
) -> anyhow::Result<bool> {
    let time = Local::now().timestamp();
    sqlx::query!(
        r#"update task set name = ?, detail_list = ?, detail_num = ?, updated_at = ? where id = ?"#,
        form.name,
        form.detail_list,
        form.detail_num,
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
        r#"update task set deleted = 1, updated_at = ? where id = ?"#,
        time,
        id
    )
    .execute(pool)
    .await?;
    Ok(true)
}

pub async fn get_by_id(id: i64, pool: &Pool) -> anyhow::Result<Task> {
    let rec = sqlx::query!(r#"SELECT * from task where id = ?"#, id as i64)
        .fetch_one(pool)
        .await?;
    let s = Task {
        id: rec.id,
        name: rec.name,
        r#type: rec.r#type as i32,
        icon_bean: rec.icon_bean,
        detail_list: rec.detail_list,
        detail_num: rec.detail_num,
        change_times: rec.change_times,
        progress: rec.progress as i32,
        remark: rec.remark,
        start_at: if rec.start_at == 0 {
            "".into()
        } else {
            Local
                .timestamp(rec.start_at as i64, 0)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        },
        end_at: if rec.end_at == 0 {
            "".into()
        } else {
            Local
                .timestamp(rec.end_at as i64, 0)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        },
        finished_at: if rec.finished_at == 0 {
            "".into()
        } else {
            Local
                .timestamp(rec.finished_at as i64, 0)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        },
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
    let rec = sqlx::query!(r#"SELECT detail_list from task where id = ?"#, id)
        .fetch_one(pool)
        .await?;
    Ok(rec.detail_list)
}