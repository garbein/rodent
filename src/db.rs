use sqlx::{MySqlPool};

pub type Pool = MySqlPool;

pub async fn create_pool(url: &str) -> Pool {
    let pool = MySqlPool::new(url).await.unwrap();
    pool
}
