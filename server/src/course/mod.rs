use actix_web::{
    get,
    http::StatusCode,
    web::{self},
    HttpResponse, Responder,
};

mod services;

mod admin;
mod professor;
mod student;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/course")
            .configure(admin::routes)
            .configure(student::routes)
            .configure(professor::routes)
            .service(
                web::scope("/professor")
                    //.service(get)
                    .service(get),
            ),
    );
}

// General

#[get("")]
async fn _get() -> impl Responder {
    match services::get_all().await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("/{id}")]
async fn get(id: web::Path<String>) -> impl Responder {
    match services::get_course(id.to_string()).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}
