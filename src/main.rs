#[macro_use]
extern crate log;
#[macro_use]
extern crate redis_async;
extern crate actix_cors;
extern crate chrono;
#[macro_use]
extern crate validator_derive;
extern crate validator;

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
mod validates;
mod errors;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_server().await
}
