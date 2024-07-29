mod services;
use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use services::DB;

use crate::shared::middlewares::admin::Admin;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/professor").wrap(Admin).service(get_professors));
}

#[get("")]
async fn get_professors(db: web::Data<DB>) -> impl Responder {
    match services::get_all(&db).await {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::build(StatusCode::from_u16(e.0).unwrap()).body(e.1),
    }
}
