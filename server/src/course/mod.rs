use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::shared::middlewares::admin::Admin;

mod services;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/course")
            .wrap(Admin)
            .service(get)
            .service(add)
            .service(delete),
    );
}

#[derive(Deserialize)]
struct Add {
    name: String,
}

#[get("")]
async fn get(db: web::Data<services::DB>) -> impl Responder {
    match services::get_all(&db).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[post("/add")]
async fn add(add: web::Form<Add>, db: web::Data<services::DB>) -> impl Responder {
    match services::create(&add.name, &db).await {
        Ok(s) => HttpResponse::Ok().json(s),
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
