use actix_web::{http::StatusCode, post, web, HttpResponse, Responder};

use crate::auth::services::signin::login;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/signin").service(signin));
}

#[derive(serde::Deserialize)]
struct SessionData {
    id: String,
    password: String,
}

#[post("")]
async fn signin(data: web::Form<SessionData>) -> impl Responder {
    match login(data.id.clone(), data.password.clone()).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, message)) => {
            HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(message)
        }
    }
}
