use crate::cache;
use crate::db::Pool;
use crate::forms::site_setting_form;
use crate::response::ApiResponse;
use crate::services::site_setting;
use crate::validates::validate;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DetailPath {
    id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    page: u32,
    size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct PaginationResult<T> {
    total: u64,
    rows: Vec<T>,
}

#[get("/settings")]
pub async fn list(
    pool: web::Data<Pool>,
    pagination: web::Query<Pagination>,
) -> Result<HttpResponse, Error> {
    let res = site_setting::list(&pool, pagination.page, pagination.size).await;
    if let Ok(t) = res {
        let pr = PaginationResult {
            total: t.0,
            rows: t.1,
        };
        Ok(HttpResponse::Ok().json(ApiResponse::succues(pr, "")))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::error("未查询到数据", ())))
    }
}

#[post("/settings")]
pub async fn create(
    pool: web::Data<Pool>,
    form: web::Json<site_setting_form::Form>,
) -> Result<HttpResponse, Error> {
    validate(&form)?;
    let res = site_setting::create(&pool, &form).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::succues((), ""))),
        Err(e) => Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

#[put("/settings/{id}")]
pub async fn update(
    pool: web::Data<Pool>,
    c: cache::Client,
    path: web::Path<DetailPath>,
    form: web::Json<site_setting_form::UpdateForm>,
) -> Result<HttpResponse, Error> {
    validate(&form)?;
    let res = site_setting::update(&pool, &c, path.id, &form).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::succues((), ""))),
        Err(e) => Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

#[delete("/settings/{id}")]
pub async fn delete(
    pool: web::Data<Pool>,
    c: cache::Client,
    path: web::Path<DetailPath>,
) -> Result<HttpResponse, Error> {
    let res = site_setting::delete(&pool, &c, path.id).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::succues((), ""))),
        Err(e) => Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

#[get("/settings/{id}")]
pub async fn view(
    pool: web::Data<Pool>,
    path: web::Path<DetailPath>,
) -> Result<HttpResponse, Error> {
    let setting = site_setting::view(&pool, path.id).await;
    if let Ok(s) = setting {
        Ok(HttpResponse::Ok().json(ApiResponse::succues(s, "")))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::error("未查询到数据", ())))
    }
}
