mod services;

use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::shared::middlewares::admin::Admin;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/faculty")
            .wrap(Admin)
            .service(faculty)
            .service(add)
            .service(delete),
    );
}

#[derive(Deserialize)]
struct Faculty {
    name: String,
}

#[get("")]
async fn faculty() -> impl Responder {
    match services::get().await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::build(StatusCode::from_u16(e.0).unwrap()).body(e.1),
    }
}

#[post("/add")]
async fn add(fac: web::Form<Faculty>) -> impl Responder {
    match services::create(&fac.name).await {
        Ok(resp) => HttpResponse::Ok().body(resp),
        Err(e) => HttpResponse::build(StatusCode::from_u16(e.0).unwrap()).body(e.1),
    }
}

// NOTE: Path says "name" but it's uses id
#[delete("/delete/{name}")]
async fn delete(fac: web::Path<Faculty>) -> impl Responder {
    match services::delete(&fac.name).await {
        Ok(resp) => HttpResponse::Ok().body(resp),
        Err(e) => HttpResponse::build(StatusCode::from_u16(e.0).unwrap()).body(e.1),
    }
}
