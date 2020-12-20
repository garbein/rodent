use crate::db::Pool;
use crate::response::ApiResponse;
use crate::services::user;
use crate::forms::user_form;
use crate::utils::token_utils;
use crate::validates::validate;
use actix_web::{get, put, web, Error, HttpRequest, HttpResponse};

/// 用户画像
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

/// 更新用户名
#[put("/user/update_username")]
pub async fn update_username(
    pool: web::Data<Pool>,
    form: web::Json<user_form::UpdateUsernameForm>,
) -> Result<HttpResponse, Error> {
    validate(&form)?;
    let res = user::update_username(&pool, &form).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::succues((), ""))),
        Err(e) => Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

/// 更新用户头像
#[put("/user/update_avatar")]
pub async fn update_avatar(
    pool: web::Data<Pool>,
    form: web::Json<user_form::UpdateAvatarForm>,
) -> Result<HttpResponse, Error> {
    validate(&form)?;
    let res = user::update_avatar(&pool, &form).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::succues((), ""))),
        Err(e) => Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}