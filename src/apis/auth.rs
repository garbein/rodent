use crate::db::Pool;
use crate::response::ApiResponse;
use actix_web::{post, web, Error, HttpResponse};
use crate::forms::auth_form;
use crate::validates::validate;
use crate::services::user;

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