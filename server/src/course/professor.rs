use actix_web::{
    get, http::StatusCode, patch, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};

use serde::Deserialize;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/professor")
            .service(courses)
            .service(update_test),
    );
}

use super::services;

#[get("")]
async fn courses(req: HttpRequest) -> impl Responder {
    let professor_id = req.extensions().get::<String>().unwrap().clone();

    match services::get_by_professor(professor_id).await {
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
