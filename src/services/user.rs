use crate::cache;
use crate::db::Pool;
use crate::forms::auth_form::*;
use crate::forms::user_form;
use crate::models::user;
use crate::models::code;
use crate::utils::token_utils::*;
use crate::utils::random;
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserRes {
    pub id: i32,
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

/// 注册
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

/// 登录
pub async fn login(pool: &web::Data<Pool>, form: &LoginForm) -> anyhow::Result<LoginRes> {
    let user_exist = user::get_by_email(&pool, &form.email).await;
    let u;
    match user_exist {
        Ok(t) => u = t,
        Err(_) => return Err(anyhow::anyhow!("user not exist")),
    };
    // 核验密码
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
    // 生成jwt token
    let token = generate_token(u.id)?;
    let res = LoginRes {
        token: token,
        user: user_res,
    };
    Ok(res)
}

/// 用户信息
pub async fn get_profile(pool: &web::Data<Pool>, user_id: i32) -> anyhow::Result<UserRes> {
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

/// 用户改名
pub async fn update_username(
    pool: &web::Data<Pool>,
    form: &user_form::UpdateUsernameForm,
) -> anyhow::Result<bool> {
    user::update_username(&pool, &form).await?;
    Ok(true)
}

/// 换头像
pub async fn update_avatar(
    pool: &web::Data<Pool>,
    form: &user_form::UpdateAvatarForm,
) -> anyhow::Result<bool> {
    user::update_avatar(&pool, &form).await?;
    Ok(true)
}

pub async fn send_code(c: &cache::Client, account: &str, engine: code::SendType) -> anyhow::Result<()> {
    let verify_code = random::rand_num(6);
    let content = format!("[闻芳随笔] 您的验证码是 {} ,10分钟内有效.请勿将验证码告诉他人", verify_code);

    let key = format!("verify_code_{}", account);
    let _ = cache::set(&c, &key, &verify_code).await;
    let _ = cache::expire(&c, &key, "600").await;

    match engine {
        code::SendType::Email => code::send_by_email(account, &content).await?,
        code::SendType::Sms => code::send_by_sms(account, &content).await?
    }

    Ok(())
}