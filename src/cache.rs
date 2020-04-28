use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::web::{Data, ServiceConfig};
use redis_async::resp::{FromResp, RespValue};

pub type Client = Data<Addr<RedisActor>>;

#[allow(dead_code)]
pub async fn expire(client: &Client, key: &str, seconds: &str) -> Result<String, String> {
    let command = resp_array!["EXPIRE", key, seconds];
    self::send(client, command).await
}

#[allow(dead_code)]
pub async fn get(client: &Client, key: &str) -> Result<String, String> {
    let command = resp_array!["GET", key];
    self::send(client, command).await
}

#[allow(dead_code)]
pub async fn set(client: &Client, key: &str, value: &str) -> Result<String, String> {
    let command = resp_array!["SET", key, value];
    self::send(client, command).await
}

#[allow(dead_code)]
pub async fn del(client: &Client, key: &str) -> Result<String, String> {
    let command = resp_array!["DEL", key];
    self::send(client, command).await
}

async fn send(client: &Client, command: RespValue) -> Result<String, String> {
    let res = client.send(Command(command)).await.map_err(|e| {
        info!("redis: {:?}", e);
        String::new()
    })?;
    match res {
        Ok(t) => {
            Ok::<String, _>(FromResp::from_resp(t).unwrap_or("".into()))
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn init_cache(cfg: &mut ServiceConfig) {
    let redis_host = std::env::var("REDIS_HOST").expect("REDIS_HOST empty");
    let redis_port = std::env::var("REDIS_PORT").expect("REDIS_PORT empty");
    let redis_addr = format!("{}:{}", redis_host, redis_port);
    let cache = RedisActor::start(redis_addr);
    cfg.data(cache);
}
