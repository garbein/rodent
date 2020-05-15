use crate::cache;
use crate::db::Pool;
use crate::response::ApiResponse;
use crate::services::site_setting;
use actix_web::{get, web, Error, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DetailPath {
    id: u64,
}

#[get("/frontend/{id}")]
pub async fn detail(
    pool: web::Data<Pool>,
    c: cache::Client,
    path: web::Path<DetailPath>,
) -> Result<HttpResponse, Error> {
    let setting = site_setting::detail(&pool, &c, path.id).await;
    if let Ok(s) = setting {
        Ok(HttpResponse::Ok().json(ApiResponse::succues(s, "")))
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::error("未查询到数据", ())))
    }
}
