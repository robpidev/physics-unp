mod services;

use actix_web::{
    delete, get,
    http::StatusCode,
    post,
    web::{self},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::shared::middlewares::admin::Admin;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/school")
            .service(
                web::scope("/admin")
                    .wrap(Admin)
                    .service(add)
                    .service(delete),
            )
            .service(get_by_id)
            .service(school),
    );
}

#[derive(Deserialize)]
struct Add {
    name: String,
    faculty_id: String,
}

#[post("/add")]
async fn add(data: web::Form<Add>) -> impl Responder {
    match services::create(&data.name, &data.faculty_id).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("")]
async fn school() -> impl Responder {
    match services::get().await {
        Ok(schools) => HttpResponse::Ok().json(schools),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("/{id}")]
async fn get_by_id(id: web::Path<String>) -> impl Responder {
    match services::get_by_id(id.to_string()).await {
        Ok(schools) => HttpResponse::Ok().json(schools),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[delete("/delete/{id}")]
async fn delete(id: web::Path<String>) -> impl Responder {
    match services::delete(&id).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}
