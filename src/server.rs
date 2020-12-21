use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};

use crate::cache;
use crate::db;
use crate::routes::init_routes;

/// 初始化server
pub async fn init_server() -> std::io::Result<()> {
    // 引入环境变量库
    dotenv::dotenv().ok();
    // 初始化log
    env_logger::init();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL empty");
    let server_host = std::env::var("SERVER_HOST").expect("SERVER_HOST empty");
    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT empty");
    let socket_addr = format!("{}:{}", server_host, server_port);
    // 创建数据库连接池
    let pool = db::create_pool(&db_url).await;
    // httpserver
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .send_wildcard()
                    .allow_any_origin()
            )
            .data(pool.clone())
            .configure(cache::init_cache)
            .wrap(middleware::Logger::default())
            .configure(init_routes)
    })
    .bind(socket_addr)?
    .run()
    .await
}
