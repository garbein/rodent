use crate::db::Pool;
use crate::response::ApiResponse;
use crate::services::user;
use crate::utils::token_utils;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};

#[get("/user/profile")]
pub async fn profile(pool: web::Data<Pool>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let token = match token_utils::extract_token(&req) {
        Some(t) => t,
        None => return Ok(HttpResponse::Ok().json(ApiResponse::error("AUTHORIZATION error", ()))),
    };
    let user_id = match token_utils::decode_token(token) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::Ok().json(ApiResponse::error("token error", ()))),
    };
    let res = user::get_profile(&pool, user_id).await;
    match res {
        Ok(t) => Ok(HttpResponse::Ok().json(ApiResponse::succues(t, ""))),
        Err(e) => Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}
