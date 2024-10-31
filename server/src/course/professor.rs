use actix_web::{
    get, http::StatusCode, patch, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};

use serde::Deserialize;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/professor")
            .wrap(Professor)
            .service(courses)
            .service(update_test)
            .service(info),
    );
}

use crate::shared::middlewares::professor::Professor;

use super::services::{self, professor};

#[get("")]
async fn courses(req: HttpRequest) -> impl Responder {
    let professor_id = req.extensions().get::<String>().unwrap().clone();

    match professor::courses(professor_id).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("/{course_id}")]
async fn info(course_id: web::Path<String>, req: HttpRequest) -> impl Responder {
    let professor_id = req.extensions().get::<String>().unwrap().clone();
    match professor::course_info(course_id.clone(), professor_id).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[derive(Deserialize)]
struct Test {
    test: u8,
    practice: u8,
}

#[patch("/{id}")]
async fn update_test(id: web::Path<String>, test: web::Form<Test>) -> impl Responder {
    match services::update_test(id.clone(), test.test, test.practice).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}
