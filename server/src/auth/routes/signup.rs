use crate::auth::services::{self, signup::DB};
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post, web, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ProfessorData {
    names: String,
    last_name1: String,
    last_name2: String,
    dni: String,
    password: String,
    gender: bool,
}

#[derive(Deserialize, Serialize)]
struct StudentData {
    code: String,
    names: String,
    last_name1: String,
    last_name2: String,
    password: String,
    gender: bool,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/signup")
            .service(hello)
            .service(professor)
            .service(student),
    );
}

#[get("")]
async fn hello() -> String {
    format!("Hello from signup")
}

#[post("/professor")]
async fn professor(professor: web::Form<ProfessorData>, db: web::Data<DB>) -> impl Responder {
    //return HttpResponse::Ok().json(professor);
    let data = services::signup::register(
        professor.dni.clone(),
        professor.password.clone(),
        "professor".to_string(),
        professor.names.clone(),
        professor.last_name1.clone(),
        professor.last_name2.clone(),
        professor.gender.clone(),
        &db,
    )
    .await;

    response(data)
}

#[post("/student")]
async fn student(student: web::Form<StudentData>, db: web::Data<DB>) -> impl Responder {
    let resp = services::signup::register(
        student.code.clone(),
        student.password.clone(),
        "student".to_string(),
        student.names.clone(),
        student.last_name1.clone(),
        student.last_name2.clone(),
        student.gender,
        &db,
    )
    .await;

    response(resp)
}

pub fn response(data: Result<String, (u16, String)>) -> impl Responder {
    match data {
        Ok(s) => HttpResponse::Ok().content_type(ContentType::json()).body(s),

        Err((code, message)) => {
            HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(message)
        }
    }
}
