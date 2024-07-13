use crate::auth::services::{self, signup::DB};
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post, web, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Professor {
    names: String,
    last_name1: String,
    last_name2: String,
    dni: String,
    password: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/signup").service(hello).service(signup));
}

#[get("")]
async fn hello() -> String {
    format!("Hello from signup")
}

#[post("/")]
async fn signup(professor: web::Form<Professor>, db: web::Data<DB>) -> impl Responder {
    //return HttpResponse::Ok().json(professor);
    return match services::signup::signup_professor(
        professor.names.clone(),
        professor.last_name1.clone(),
        professor.last_name2.clone(),
        professor.dni.clone(),
        professor.password.clone(),
        &db,
    )
    .await
    {
        Ok(professor) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(professor),

        Err((code, message)) => {
            HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(message)
        }
    };
}
