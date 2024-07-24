mod services;

use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::shared::middlewares::admin;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/school")
            .wrap(admin::Admin)
            .service(school)
            .service(add)
            .service(delete),
    );
}

#[derive(Deserialize)]
struct Add {
    name: String,
    faculty_id: String,
}

#[post("/add")]
async fn add(data: web::Form<Add>, db: web::Data<services::DB>) -> impl Responder {
    match services::create(&data.name, &data.faculty_id, &db).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("")]
async fn school(db: web::Data<services::DB>) -> impl Responder {
    match services::get(&db).await {
        Ok(schools) => HttpResponse::Ok().json(schools),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[delete("/delete/{id}")]
async fn delete(id: web::Path<String>, db: web::Data<services::DB>) -> impl Responder {
    match services::delete(&id, &db).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}
