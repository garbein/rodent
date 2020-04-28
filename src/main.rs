#[macro_use]
extern crate log;
#[macro_use]
extern crate redis_async;
extern crate actix_cors;
extern crate chrono;

use crate::server::init_server;

mod apis;
mod cache;
mod db;
mod models;
mod response;
mod routes;
mod server;
mod services;
mod forms;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_server().await
}
