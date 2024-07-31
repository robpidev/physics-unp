mod services;

use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use services::DB;

use crate::shared::middlewares::admin::Admin;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/student")
            .wrap(Admin)
            .service(get_by_school)
            .service(get_by_course),
    );
}

#[get("/school/{school}")]
async fn get_by_school(_data: web::Path<String>, _db: web::Data<DB>) -> impl Responder {
    HttpResponse::Ok().body("Hello from student")
}

#[get("/course/{course}")]
async fn get_by_course(course_id: web::Path<String>, db: web::Data<DB>) -> impl Responder {
    match services::students_by_course(&course_id, &db).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((n, e)) => HttpResponse::build(StatusCode::from_u16(n).unwrap()).body(e),
    }
}
