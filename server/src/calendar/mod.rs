use actix_web::{delete, get, http::StatusCode, web, HttpResponse, Responder};
use services::DB;

use crate::shared::middlewares::admin::Admin;
mod services;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/calendar")
            .wrap(Admin)
            .service(get)
            .service(delete),
    );
}

#[get("")]
async fn get(db: web::Data<DB>) -> impl Responder {
    match services::get_datetimes(&db).await {
        Ok(dt) => return HttpResponse::Ok().json(dt),
        Err((n, e)) => return HttpResponse::build(StatusCode::from_u16(n).unwrap()).body(e),
    }
}

#[delete("/{id}")]
async fn delete(id: web::Path<String>, db: web::Data<DB>) -> impl Responder {
    match services::delete(&id, &db).await {
        Ok(_) => return HttpResponse::Ok().finish(),
        Err((n, e)) => return HttpResponse::build(StatusCode::from_u16(n).unwrap()).body(e),
    }
}
