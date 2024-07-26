use actix_web::{
    delete, get, http::StatusCode, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use serde::Deserialize;

use crate::shared::middlewares::{admin::Admin, student::StudentAuth};

mod services;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                web::scope("/professor/course")
                    .wrap(Admin)
                    .service(add)
                    .service(delete)
                    .service(get)
                    .service(get_by_school)
                    .service(register_by_id)
                    .service(unregister),
            )
            .service(
                web::scope("/course")
                    .wrap(StudentAuth)
                    .service(get)
                    .service(get_by_school)
                    .service(register),
            ),
    );
}

#[derive(Deserialize)]
struct Add {
    name: String,
    places: u16,
    school_id: String,
}

#[get("")]
async fn get(db: web::Data<services::DB>) -> impl Responder {
    match services::get_all(&db).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[post("")]
async fn add(add: web::Form<Add>, db: web::Data<services::DB>) -> impl Responder {
    match services::create(&add.name, add.places, &add.school_id, &db).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[delete("/{id}")]
async fn delete(id: web::Path<String>, db: web::Data<services::DB>) -> impl Responder {
    match services::delete(&id, &db).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("/{school_id}")]
async fn get_by_school(
    school_id: web::Path<String>,
    db: web::Data<services::DB>,
) -> impl Responder {
    match services::get_by_school(&school_id, &db).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[post("/register/{course_id}")]
async fn register(
    course_id: web::Path<String>,
    req: HttpRequest,
    db: web::Data<services::DB>,
) -> impl Responder {
    let student_id = match req.extensions().get::<String>() {
        Some(id) => id.clone(),
        None => return HttpResponse::InternalServerError().body("No Student Id"),
    };

    match services::register(&course_id, &student_id, &db).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[derive(Deserialize)]
struct Enroll {
    course_id: String,
    student_id: String,
}

#[post("/enroll")]
async fn register_by_id(data: web::Form<Enroll>, db: web::Data<services::DB>) -> impl Responder {
    match services::register(&data.course_id, &data.student_id, &db).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[delete("/unenroll/")]
async fn unregister(data: web::Form<Enroll>, db: web::Data<services::DB>) -> impl Responder {
    match services::unregister(&data.course_id, &data.student_id, &db).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}
