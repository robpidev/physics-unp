use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use services::DB;
mod services;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/calendar").service(get));
}

#[get("")]
async fn get(db: web::Data<DB>) -> impl Responder {
    match services::get_datetimes(&db).await {
        Ok(dt) => return HttpResponse::Ok().json(dt),
        Err((n, e)) => return HttpResponse::build(StatusCode::from_u16(n).unwrap()).body(e),
    }
}
