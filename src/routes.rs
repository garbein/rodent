use actix_web::web;
use crate::apis::{
    index::{index},
    frontend,
};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(index)
    .service(frontend::create)
    .service(frontend::update)
    .service(frontend::delete)
    .service(frontend::detail);
}