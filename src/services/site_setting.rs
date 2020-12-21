/// 配置服务
use crate::cache;
use crate::db::Pool;
use crate::forms::site_setting_form;
use crate::models::site_setting;
use actix_web::web;
use chrono::prelude::*;
use serde_json;

/// 列表
pub async fn list(
    pool: &web::Data<Pool>,
    page: i32,
    size: i32,
) -> anyhow::Result<(i64, Vec<site_setting::Setting>)> {
    site_setting::get_all(&pool, page, size).await
}

/// 创建
pub async fn create(pool: &web::Data<Pool>, form: &site_setting_form::Form) -> anyhow::Result<i64> {
    site_setting::create(form, &pool).await
}

/// 更新
pub async fn update(
    pool: &web::Data<Pool>,
    c: &cache::Client,
    id: i64,
    form: &site_setting_form::UpdateForm,
) -> anyhow::Result<bool> {
    site_setting::update(id, &form, &pool).await?;
    let key = format!("site_setting:{}", id);
    let _ = cache::del(&c, &key).await;
    Ok(true)
}

/// 删除
pub async fn delete(pool: &web::Data<Pool>, c: &cache::Client, id: i64) -> anyhow::Result<bool> {
    site_setting::delete(id, &pool).await?;
    let key = format!("site_setting:{}", id);
    let _ = cache::del(&c, &key).await;
    Ok(true)
}

/// 详情
pub async fn view(
    pool: &web::Data<Pool>,
    id: i64,
) -> anyhow::Result<site_setting::Setting> {
    site_setting::get_by_id(id, &pool).await
}

/// 前端接口
pub async fn detail(
    pool: &web::Data<Pool>,
    c: &cache::Client,
    id: i64,
) -> anyhow::Result<serde_json::Value> {
    let key = format!("site_setting:{}", id);
    let setting_cache = cache::get(&c, &key).await;
    if let Ok(setting_cache) = setting_cache {
        if setting_cache.len() > 0 {
            return handle_setting(&setting_cache);
        }
    }
    let setting = site_setting::get_content_by_id(id, &pool).await?;
    if setting.len() > 0 {
        let _ = cache::set(&c, &key, &setting).await;
        //let _ = cache::expire(&c, &key, "86400").await;
        handle_setting(&setting)
    } else {
        Err(anyhow::anyhow!("not found"))
    }
}

/// 处理配置
/// 主要处理时间格式
fn handle_setting(setting: &str) -> anyhow::Result<serde_json::Value> {
    let obj: serde_json::Value = serde_json::from_str(setting)?;
    let name_start_time = obj["name_start_time"].as_str();
    let name_end_time = obj["name_end_time"].as_str();
    let st: i64 = match name_start_time {
        Some(t) => {
            if t.len() > 0 {
                let dt = Local.datetime_from_str(t, "%Y-%m-%d %H:%M:%S")?;
                dt.timestamp()
            } else {
                0
            }
        }
        None => 0,
    };
    let et: i64 = match name_end_time {
        Some(t) => {
            if t.len() > 0 {
                let dt = Local.datetime_from_str(t, "%Y-%m-%d %H:%M:%S")?;
                dt.timestamp()
            } else {
                0
            }
        }
        None => 0,
    };
    let t = Local::now().timestamp();
    if (st > 0 && t < st) || (et > 0 && t > et) {
        return Err(anyhow::anyhow!("setting not in validity"));
    }
    Ok(obj)
}
