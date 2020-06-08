use actix_web::web;
use crate::apis::{
    index,
    settings,
    frontend,
    auth,
    user,
    task,
    suggestion,
};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(index::index)
    .service(index::check_update)
    .service(auth::register)
    .service(auth::login)
    .service(auth::get_email_code)
    .service(auth::get_sms_code)
    .service(user::profile)
    .service(user::update_username)
    .service(user::update_avatar)
    .service(settings::list)
    .service(settings::create)
    .service(settings::update)
    .service(settings::delete)
    .service(settings::view)
    .service(frontend::detail)
    .service(task::list)
    .service(task::create)
    .service(task::update)
    .service(task::delete)
    .service(task::view)
    .service(task::detail)
    .service(suggestion::list)
    .service(suggestion::create)
    .service(suggestion::update)
    .service(suggestion::delete)
    .service(suggestion::view)
    .service(suggestion::detail);
}