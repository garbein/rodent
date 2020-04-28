use crate::cache;
use crate::db::Pool;
use crate::forms::site_setting_form;
use crate::response::ApiResponse;
use crate::services::site_setting;
use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DetailPath {
    name: String,
}

#[post("/frontend")]
pub async fn create(
    pool: web::Data<Pool>,
    form: web::Json<site_setting_form::Form>,
) -> Result<HttpResponse> {
    let res = site_setting::create(&pool, &form).await;
    if res {
        Ok(HttpResponse::Ok().json(ApiResponse::succues((), "")))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::error("操作失败", ())))
    }
}

#[put("/frontend/{name}")]
pub async fn update(
    pool: web::Data<Pool>,
    c: cache::Client,
    path: web::Path<DetailPath>,
    form: web::Json<site_setting_form::Form>,
) -> Result<HttpResponse> {
    let res = site_setting::update(&pool, &c, &path.name, &form).await;
    if res {
        Ok(HttpResponse::Ok().json(ApiResponse::succues((), "")))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::error("操作失败", ())))
    }
}

#[delete("/frontend/{name}")]
pub async fn delete(
    pool: web::Data<Pool>,
    c: cache::Client,
    path: web::Path<DetailPath>,
) -> Result<HttpResponse> {
    let res = site_setting::delete(&pool, &c, &path.name).await;
    if res {
        Ok(HttpResponse::Ok().json(ApiResponse::succues((), "")))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::error("操作失败", ())))
    }
}

#[get("/frontend/{name}")]
pub async fn detail(
    pool: web::Data<Pool>,
    c: cache::Client,
    path: web::Path<DetailPath>,
) -> Result<HttpResponse> {
    let setting = site_setting::detail(&pool, &c, &path.name).await;
    if let Ok(s) = setting {
        Ok(HttpResponse::Ok().json(ApiResponse::succues(s, "")))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::error("未查询到数据", ())))
    }
}
