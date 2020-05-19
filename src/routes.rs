use actix_web::web;
use crate::apis::{
    index::{index},
    settings,
    frontend,
    auth,
    user,
};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(index)
    .service(auth::register)
    .service(auth::login)
    .service(user::profile)
    .service(settings::list)
    .service(settings::create)
    .service(settings::update)
    .service(settings::delete)
    .service(settings::view)
    .service(frontend::detail);
}