use crate::cache;
use crate::db::Pool;
use crate::forms::site_setting_form;
use crate::response::ApiResponse;
use crate::services::site_setting;
use actix_web::{delete, get, post, put, web, HttpResponse, Error};
use serde::Deserialize;
use crate::validates::validate;

#[derive(Deserialize)]
pub struct DetailPath {
    name: String,
}

#[post("/frontend")]
pub async fn create(
    pool: web::Data<Pool>,
    form: web::Json<site_setting_form::Form>,
) -> Result<HttpResponse, Error> {
    validate(&form)?;
    let res = site_setting::create(&pool, &form).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::succues((), ""))),
        Err(e) =>  Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

#[put("/frontend/{name}")]
pub async fn update(
    pool: web::Data<Pool>,
    c: cache::Client,
    path: web::Path<DetailPath>,
    form: web::Json<site_setting_form::UpdateForm>,
) -> Result<HttpResponse, Error> {
    validate(&form)?;
    let res = site_setting::update(&pool, &c, &path.name, &form).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::succues((), ""))),
        Err(e) =>  Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

#[delete("/frontend/{name}")]
pub async fn delete(
    pool: web::Data<Pool>,
    c: cache::Client,
    path: web::Path<DetailPath>,
) -> Result<HttpResponse, Error> {
    let res = site_setting::delete(&pool, &c, &path.name).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::succues((), ""))),
        Err(e) =>  Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

#[get("/frontend/{name}")]
pub async fn detail(
    pool: web::Data<Pool>,
    c: cache::Client,
    path: web::Path<DetailPath>,
) -> Result<HttpResponse, Error> {
    let setting = site_setting::detail(&pool, &c, &path.name).await;
    if let Ok(s) = setting {
        Ok(HttpResponse::Ok().json(ApiResponse::succues(s, "")))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::error("未查询到数据", ())))
    }
}
