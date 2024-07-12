use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Professor {
    names: String,
    last_name1: String,
    last_name2: String,
    dni: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/signup").service(hello).service(signup));
}

#[get("")]
async fn hello() -> String {
    format!("Hello from signup")
}

#[post("/")]
async fn signup(professor: web::Form<Professor>) -> impl Responder {
    HttpResponse::Ok().json(professor)
}
