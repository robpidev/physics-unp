use actix_web::{delete, get, patch, post, web};
use actix_web::{http::StatusCode, HttpResponse, Responder};
use serde::Deserialize;

use crate::shared::middlewares::admin::Admin;

use super::services::{self, admin};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .wrap(Admin)
            .service(update_places)
            .service(get_by_school)
            .service(create)
            .service(delete)
            .service(enroll)
            .service(unenroll)
            .service(course_info)
            .service(asign)
            .service(unasign),
    );
}

#[derive(Deserialize)]
struct NewCourse {
    name: String,
    places: u16,
    school_id: String,
}

#[derive(Deserialize)]
struct UpdatePlaces {
    course_id: String,
    places: u16,
}

#[get("/{id}")]
async fn course_info(id: web::Path<String>) -> impl Responder {
    match admin::course(&id).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[post("")]
async fn create(add: web::Form<NewCourse>) -> impl Responder {
    match admin::create(&add.name, add.places, &add.school_id).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[delete("/{id}")]
async fn delete(id: web::Path<String>) -> impl Responder {
    match services::delete(&id).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[patch("/places")]
async fn update_places(course: web::Json<UpdatePlaces>) -> impl Responder {
    match services::update_places(course.course_id.clone(), course.places).await {
        Ok(s) => HttpResponse::Ok().body(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("/school/{school_id}")]
async fn get_by_school(school_id: web::Path<String>) -> impl Responder {
    match services::get_by_school(&school_id).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[derive(Deserialize)]
struct Asign {
    course_id: String,
    user_id: String,
    role: String,
}

// professor
#[post("professor/asign")]
async fn asign(data: web::Form<Asign>) -> impl Responder {
    match admin::asign_professor(
        data.course_id.clone(),
        data.user_id.clone(),
        data.role.clone(),
    )
    .await
    {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[derive(Deserialize)]
struct DesasingInfo {
    course_id: String,
    user_id: String,
}

#[delete("/professor/asign")]
async fn unasign(data: web::Form<DesasingInfo>) -> impl Responder {
    match admin::desasign_professor(&data.course_id, &data.user_id).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[derive(Deserialize)]
struct Enroll {
    course_id: String,
    student_id: String,
}

// Test

// Enroll
#[post("/enroll")]
async fn enroll(data: web::Form<Enroll>) -> impl Responder {
    match services::enroll::enroll(data.student_id.clone(), data.course_id.clone()).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[delete("/enroll")]
async fn unenroll(data: web::Form<Enroll>) -> impl Responder {
    match services::unregister(&data.course_id, &data.student_id).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}
