use crate::db::Pool;
use crate::forms::auth_form::*;
use crate::models::user;
use crate::utils::token_utils::*;
use actix_web::web;
use serde::{Deserialize, Serialize};

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
    let reg_form = RegisterForm {
        password: password,
        ..form.clone()
    };
    let user_id = user::create(&pool, &reg_form).await?;
    let user_res = UserRes {
        id: user_id,
        name: form.name.to_string(),
        email: form.email.to_string(),
        role: String::from("user"),
        verified: true,
    };
    let token = generate_token(user_id)?;
    let res = LoginRes {
        token: token,
        user: user_res,
    };
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
    let user_res = UserRes {
        id: u.id,
        name: u.name.to_string(),
        email: u.email.to_string(),
        role: String::from("user"),
        verified: true,
    };
    let token = generate_token(u.id)?;
    let res = LoginRes {
        token: token,
        user: user_res,
    };
    Ok(res)
}

pub async fn get_profile(pool: &web::Data<Pool>, user_id: u32) -> anyhow::Result<UserRes> {
    let u = user::get_by_id(&pool, user_id).await?;
    let user_res = UserRes {
        id: u.id,
        name: u.name.to_string(),
        email: u.email.to_string(),
        role: String::from("user"),
        verified: true,
    };
    Ok(user_res)
}
