use actix_web::{get, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    "drop the world"
}

#[get("/app/check_update")]
pub async fn check_update() -> impl Responder {
    "drop the world"
}