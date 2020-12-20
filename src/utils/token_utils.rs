/// jwt
use chrono::prelude::*;
use chrono::Duration;
use jsonwebtoken::{Header, EncodingKey, DecodingKey};
use rand::{thread_rng, RngCore};
use serde::{Deserialize, Serialize};
use actix_web::{HttpRequest};
use crate::utils::constants;

pub const SECRET_KEY: &str = "5432fin4303f00994qq0afgj44e400s";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub uid: i32,
    pub exp: i64,
}

/// 加密密码
pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = generate_random_salt();
    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &argon2::Config::default())?;
    Ok(hash)
}

/// 生成密码salt
fn generate_random_salt() -> [u8; 16] {
    let mut salt = [0; 16];
    thread_rng().fill_bytes(&mut salt);
    salt
}

/// 生成token
pub fn generate_token(user_id: i32) -> anyhow::Result<String> {
    let exp = Local::now() + Duration::hours(24);
    let token = jsonwebtoken::encode(
        &Header::default(),
        &TokenClaims {
            uid: user_id,
            exp: exp.timestamp(),
        },
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    )?;
    Ok(token)
}

/// 提取token中用户信息
pub fn extract_token(req: &HttpRequest) -> Option<&str> {
    if let Some(header) = req.headers().get(constants::AUTHORIZATION) {
        if let Ok(authorization) = header.to_str() {
            return Some(&authorization[7..authorization.len()]);
        }
    }
    None
}

/// 编码token
pub fn decode_token(token: &str) -> anyhow::Result<i32> {
    let data = jsonwebtoken::decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY.as_ref()),
        &jsonwebtoken::Validation::default(),
    )?;
    Ok(data.claims.uid)
}