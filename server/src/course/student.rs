use actix_web::{
    get, http::StatusCode, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};

use serde::Deserialize;

use super::services::{self, student};

use crate::shared::middlewares::student::StudentAuth;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/student")
            .wrap(StudentAuth)
            .service(avilables)
            .service(enroll)
            .service(enrolled),
    );
}

// student
#[get("/avilables")]
async fn avilables(req: HttpRequest) -> impl Responder {
    let student_id = req.extensions().get::<String>().unwrap().clone();

    match services::student::avilables(student_id).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[derive(Deserialize)]
struct Enroll {
    course_id: String,
    ocupated_groups: Vec<u8>,
}

#[post("/enroll")]
async fn enroll(data: web::Json<Enroll>, req: HttpRequest) -> impl Responder {
    let student_id = match req.extensions().get::<String>() {
        Some(id) => id.clone(),
        None => return HttpResponse::InternalServerError().body("No Student Id"),
    };

    match student::enroll(
        student_id,
        data.course_id.clone(),
        data.ocupated_groups.clone(),
    )
    .await
    {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("/enrolled")]
async fn enrolled(req: HttpRequest) -> impl Responder {
    let student_id = req.extensions().get::<String>().unwrap().clone();
    match student::enrolled(student_id).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}
