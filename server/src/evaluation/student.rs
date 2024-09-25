use actix_web::{get, http::StatusCode, web, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::shared::middlewares::student::StudentAuth;

use super::services::student;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/student")
            .wrap(StudentAuth)
            .service(course_evaluation),
    );
}

#[get("/{course_id}")]
async fn course_evaluation(course_id: web::Path<String>, req: HttpRequest) -> impl Responder {
    let student_id = req.extensions().get::<String>().unwrap().clone();
    match student::evaluation(&student_id, &course_id).await {
        Ok(c) => HttpResponse::Ok().json(c),
        Err((code, e)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(e),
    }
}
