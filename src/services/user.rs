use crate::db::Pool;
use crate::forms::auth_form::*;
use actix_web::web;
use crate::models::user;
use crate::utils::token_utils::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserRes {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRes {
    pub token: String,
    pub user: UserRes,
}


pub async fn register(pool: &web::Data<Pool>, form: &RegisterForm) -> anyhow::Result<LoginRes> {
    let user_exist = user::get_by_email(&pool, &form.email).await;
    if user_exist.is_ok() {
        return Err(anyhow::anyhow!("email exist"));
    }
    let password = hash_password(&form.password)?;
    let reg_form = RegisterForm{password: password, ..form.clone()};
    let user_id = user::create(&pool, &reg_form).await?;
    let user_res = UserRes{
        id: user_id,
        name: form.name.to_string(),
        email: form.email.to_string(),
        role: String::from("user"),
        verified: true,
    };
    let token = generate_token(user_id)?;
    let res = LoginRes {token: token, user: user_res};
    Ok(res)
}

pub async fn login(pool: &web::Data<Pool>, form: &LoginForm) -> anyhow::Result<LoginRes> {
    let user_exist = user::get_by_email(&pool, &form.email).await;
    let u;
    match user_exist {
        Ok(t) => u = t,
        Err(_) => return Err(anyhow::anyhow!("user not exist")), 
    };
    let valid = argon2::verify_encoded(&u.password, &form.password.as_bytes())?;
    if !valid {
        return Err(anyhow::anyhow!("password error"));
    }
    let user_res = UserRes{
        id: u.id,
        name: u.name.to_string(),
        email: u.email.to_string(),
        role: String::from("user"),
        verified: true,
    };
    let token = generate_token(u.id)?;
    let res = LoginRes {token: token, user: user_res};
    Ok(res)
}