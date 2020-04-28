use crate::cache;
use crate::db::Pool;
use crate::forms::site_setting_form;
use crate::models::site_setting;
use actix_web::web;
use chrono::prelude::*;
use serde_json;

pub async fn create(pool: &web::Data<Pool>, form: &site_setting_form::Form) -> bool {
    let id = site_setting::create(form, &pool).await;
    if id == 0 {
        false
    } else {
        true
    }
}

pub async fn update(
    pool: &web::Data<Pool>,
    c: &cache::Client,
    name: &str,
    form: &site_setting_form::Form,
) -> bool {
    let update_ok = site_setting::update(name, &form, &pool).await;
    if update_ok {
        let key = format!("site_setting:{}", name);
        let _ = cache::del(&c, &key).await;
    }
    update_ok
}

pub async fn delete(pool: &web::Data<Pool>, c: &cache::Client, name: &str) -> bool {
    let delete_ok = site_setting::delete(name, &pool).await;
    if delete_ok {
        let key = format!("site_setting:{}", name);
        let _ = cache::del(&c, &key).await;
    }
    delete_ok
}

pub async fn detail(
    pool: &web::Data<Pool>,
    c: &cache::Client,
    name: &str,
) -> Result<serde_json::Value, String> {
    let key = format!("site_setting:{}", name);
    let setting_cache = cache::get(&c, &key).await;
    if let Ok(setting_cache) = setting_cache {
        if false && setting_cache.len() > 0 {
            return match handle_setting(&setting_cache) {
                Some(t) => Ok(t),
                None => Err(String::from("not found")),
            };
        }
    }
    let setting = site_setting::get_by_name(name, &pool).await;
    if setting.len() > 0 {
        let _ = cache::set(&c, &key, &setting).await;
        let _ = cache::expire(&c, &key, "86400").await;
        return match handle_setting(&setting) {
            Some(t) => Ok(t),
            None => Err(String::from("not found")),
        };
    } else {
        Err(String::from("not found"))
    }
}

fn handle_setting(setting: &str) -> Option<serde_json::Value> {
    let obj: serde_json::Value = serde_json::from_str(setting).unwrap();
    let name_start_time = obj["name_start_time"].as_str();
    let name_end_time = obj["name_end_time"].as_str();
    let st: i64 = match name_start_time {
        Some(t) => {
            if t.len() > 0 {
                let dt = Local.datetime_from_str(t, "%Y-%m-%d %H:%M:%S").unwrap();
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
                let dt = Local.datetime_from_str(t, "%Y-%m-%d %H:%M:%S").unwrap();
                dt.timestamp()
            } else {
                0
            }
        }
        None => 0,
    };
    let t = Local::now().timestamp();
    if st > 0 && t < st {
        return None;
    }
    if et > 0 && t > et {
        return None;
    }
    Some(obj)
}
