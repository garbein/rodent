use crate::cache;
use crate::db::Pool;
use crate::forms::site_setting_form;
use crate::models::site_setting;
use actix_web::web;
use chrono::prelude::*;
use serde_json;

pub async fn create(pool: &web::Data<Pool>, form: &site_setting_form::Form) -> anyhow::Result<u64> {
    site_setting::create(form, &pool).await
}

pub async fn update(
    pool: &web::Data<Pool>,
    c: &cache::Client,
    name: &str,
    form: &site_setting_form::UpdateForm,
) -> anyhow::Result<bool> {
    site_setting::update(name, &form, &pool).await?;
    let key = format!("site_setting:{}", name);
    let _ = cache::del(&c, &key).await;
    Ok(true)
}

pub async fn delete(pool: &web::Data<Pool>, c: &cache::Client, name: &str) -> anyhow::Result<bool> {
    site_setting::delete(name, &pool).await?;
    let key = format!("site_setting:{}", name);
    let _ = cache::del(&c, &key).await;
    Ok(true)
}

pub async fn detail(
    pool: &web::Data<Pool>,
    c: &cache::Client,
    name: &str,
) -> anyhow::Result<serde_json::Value> {
    let key = format!("site_setting:{}", name);
    let setting_cache = cache::get(&c, &key).await;
    if let Ok(setting_cache) = setting_cache {
        if setting_cache.len() > 0 {
            return handle_setting(&setting_cache);
        }
    }
    let setting = site_setting::get_by_name(name, &pool).await?;
    if setting.len() > 0 {
        let _ = cache::set(&c, &key, &setting).await;
        let _ = cache::expire(&c, &key, "86400").await;
        handle_setting(&setting)
    } else {
        Err(anyhow::anyhow!("not found"))
    }
}

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
