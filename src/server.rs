use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};

use crate::cache;
use crate::db;
use crate::routes::init_routes;

pub async fn init_server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL empty");
    let server_host = std::env::var("SERVER_HOST").expect("SERVER_HOST empty");
    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT empty");
    let socket_addr = format!("{}:{}", server_host, server_port);
    let pool = db::create_pool(&db_url).await;
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .finish()
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
