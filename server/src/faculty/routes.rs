use crate::faculty::services;
use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::shared::middlewares::admin::Admin;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/faculty")
            .service(faculty)
            .wrap(Admin)
            .service(add)
            .service(delete),
    );
}

#[derive(Deserialize)]
struct Faculty {
    name: String,
}

#[get("")]
async fn faculty(fac: web::Query<Faculty>, db: web::Data<services::DB>) -> impl Responder {
    match services::get(&fac.name, &db).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::build(StatusCode::from_u16(e.0).unwrap()).body(e.1),
    }
}

#[post("/add")]
async fn add(fac: web::Form<Faculty>, db: web::Data<services::DB>) -> impl Responder {
    match services::create(&fac.name, &db).await {
        Ok(resp) => HttpResponse::Ok().body(resp),
        Err(e) => HttpResponse::build(StatusCode::from_u16(e.0).unwrap()).body(e.1),
    }
}

// NOTE: Path says "name" but it's uses id
#[delete("/delete/{name}")]
async fn delete(fac: web::Path<Faculty>, db: web::Data<services::DB>) -> impl Responder {
    match services::delete(&fac.name, &db).await {
        Ok(resp) => HttpResponse::Ok().body(resp),
        Err(e) => HttpResponse::build(StatusCode::from_u16(e.0).unwrap()).body(e.1),
    }
}
