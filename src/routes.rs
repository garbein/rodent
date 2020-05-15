use actix_web::web;
use crate::apis::{
    index::{index},
    settings,
};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(index)
    .service(settings::list)
    .service(settings::create)
    .service(settings::update)
    .service(settings::delete)
    .service(settings::detail);
}