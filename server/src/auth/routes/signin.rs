use actix_web::{post, web, Responder};

use crate::auth::services::signin::{login, DB};

use super::response;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/signin").service(signin));
}

#[derive(serde::Deserialize)]
struct SessionData {
    id: String,
    password: String,
}

#[post("")]
async fn signin(data: web::Form<SessionData>, db: web::Data<DB>) -> impl Responder {
    response(login(data.id.clone(), data.password.clone(), &db).await)
}
