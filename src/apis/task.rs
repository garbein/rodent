use crate::cache;
use crate::db::Pool;
use crate::forms::task_form;
use crate::response::ApiResponse;
use crate::services::task;
use crate::validates::validate;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DetailPath {
    id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    page: i32,
    size: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PaginationResult<T> {
    total: i64,
    rows: Vec<T>,
}

#[get("/task")]
pub async fn list(
    pool: web::Data<Pool>,
    pagination: web::Query<Pagination>,
) -> Result<HttpResponse, Error> {
    let res = task::list(&pool, pagination.page, pagination.size).await;
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

#[post("/task")]
pub async fn create(
    pool: web::Data<Pool>,
    form: web::Json<task_form::Form>,
) -> Result<HttpResponse, Error> {
    validate(&form)?;
    let res = task::create(&pool, &form).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::succues((), ""))),
        Err(e) => Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

#[put("/task/{id}")]
pub async fn update(
    pool: web::Data<Pool>,
    c: cache::Client,
    path: web::Path<DetailPath>,
    form: web::Json<task_form::UpdateForm>,
) -> Result<HttpResponse, Error> {
    validate(&form)?;
    let res = task::update(&pool, &c, path.id, &form).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::succues((), ""))),
        Err(e) => Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

#[delete("/task/{id}")]
pub async fn delete(
    pool: web::Data<Pool>,
    c: cache::Client,
    path: web::Path<DetailPath>,
) -> Result<HttpResponse, Error> {
    let res = task::delete(&pool, &c, path.id).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::succues((), ""))),
        Err(e) => Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

#[get("/task/{id}")]
pub async fn view(
    pool: web::Data<Pool>,
    path: web::Path<DetailPath>,
) -> Result<HttpResponse, Error> {
    let setting = task::view(&pool, path.id).await;
    if let Ok(s) = setting {
        Ok(HttpResponse::Ok().json(ApiResponse::succues(s, "")))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::error("未查询到数据", ())))
    }
}

#[get("/task_detail/{id}")]
pub async fn detail(
    pool: web::Data<Pool>,
    c: cache::Client,
    path: web::Path<DetailPath>,
) -> Result<HttpResponse, Error> {
    let setting = task::detail(&pool, &c, path.id).await;
    if let Ok(s) = setting {
        Ok(HttpResponse::Ok().json(ApiResponse::succues(s, "")))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::error("未查询到数据", ())))
    }
}
