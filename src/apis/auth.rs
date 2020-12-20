use crate::cache;
use crate::db::Pool;
use crate::response::ApiResponse;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use crate::forms::auth_form;
use crate::validates::validate;
use crate::services::user;
use crate::models::code;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    email: String,
    mobile: String,
}

/// 用户注册
#[post("/register")]
pub async fn register(
    pool: web::Data<Pool>,
    form: web::Json<auth_form::RegisterForm>,
) -> Result<HttpResponse, Error> {
    validate(&form)?;
    let res = user::register(&pool, &form).await;
    match res {
        Ok(t) => Ok(HttpResponse::Ok().json(ApiResponse::succues(t, ""))),
        Err(e) => Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

/// 用户登录
#[post("/login")]
pub async fn login(
    pool: web::Data<Pool>,
    form: web::Json<auth_form::LoginForm>,
) -> Result<HttpResponse, Error> {
    validate(&form)?;
    let res = user::login(&pool, &form).await;
    match res {
        Ok(t) => Ok(HttpResponse::Ok().json(ApiResponse::succues(t, ""))),
        Err(e) => Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

/// 发送邮件验证码
#[get("/get_email_code")]
pub async fn get_email_code(
    c: cache::Client,
    account: web::Query<Account>,
) -> Result<HttpResponse, Error> {
    let res = user::send_code(&c, &account.email, code::SendType::Email).await;
    match res {
        Ok(t) => Ok(HttpResponse::Ok().json(ApiResponse::succues(t, ""))),
        Err(e) => Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

/// 发送短信验证码
#[get("/get_sms_code")]
pub async fn get_sms_code(
    c: cache::Client,
    account: web::Query<Account>,
) -> Result<HttpResponse, Error> {
    let res = user::send_code(&c, &account.mobile, code::SendType::Sms).await;
    match res {
        Ok(t) => Ok(HttpResponse::Ok().json(ApiResponse::succues(t, ""))),
        Err(e) => Ok(HttpResponse::Ok().json(ApiResponse::error(&e.to_string(), ()))),
    }
}

/// 验证验证码
#[post("/check_code")]
pub async fn check_code() -> impl Responder {
    "ok"
}