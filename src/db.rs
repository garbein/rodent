#[allow(unused_imports)]
use sqlx::{MySqlPool, SqlitePool, PgPool};

#[cfg(feature = "postgres")]
pub type Pool = PgPool;

#[cfg(feature = "sqlite")]
pub type Pool = SqlitePool;

#[cfg(feature = "mysql")]
pub type Pool = MySqlPool;

pub async fn create_pool(url: &str) -> Pool {    
    let pool = Pool::new(url).await.unwrap();    
    pool
}
