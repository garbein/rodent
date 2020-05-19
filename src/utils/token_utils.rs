use chrono::prelude::*;
use chrono::Duration;
use jsonwebtoken::{Header, EncodingKey};
use rand::{thread_rng, RngCore};
use serde::{Deserialize, Serialize};

pub const SECRET_KEY: &str = "5432fin4303f00994qq0afgj44e400s";

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub uid: u32,
    pub exp: i64,
}

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = generate_random_salt();
    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &argon2::Config::default())?;
    Ok(hash)
}

fn generate_random_salt() -> [u8; 16] {
    let mut salt = [0; 16];
    thread_rng().fill_bytes(&mut salt);
    salt
}

pub fn generate_token(user_id: u32) -> anyhow::Result<String> {
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
