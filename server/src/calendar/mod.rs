use actix_web::{
    delete, get,
    http::StatusCode,
    post,
    web::{self},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::shared::middlewares::admin::Admin;
mod services;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/calendar")
            .wrap(Admin)
            .service(get)
            .service(delete)
            .service(create),
    );
}

#[get("")]
async fn get() -> impl Responder {
    match services::get_datetimes().await {
        Ok(dt) => return HttpResponse::Ok().json(dt),
        Err((n, e)) => return HttpResponse::build(StatusCode::from_u16(n).unwrap()).body(e),
    }
}

#[delete("/{id}")]
async fn delete(id: web::Path<String>) -> impl Responder {
    match services::delete(id.to_string()).await {
        Ok(_) => return HttpResponse::Ok().finish(),
        Err((n, e)) => return HttpResponse::build(StatusCode::from_u16(n).unwrap()).body(e),
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Add {
    end: String,
    todo: String,
}

#[post("")]
async fn create(add: web::Form<Add>) -> impl Responder {
    match services::add(add.todo.clone(), add.end.clone()).await {
        Ok(s) => return HttpResponse::Ok().json(s),
        Err((n, e)) => return HttpResponse::build(StatusCode::from_u16(n).unwrap()).body(e),
    }
}
