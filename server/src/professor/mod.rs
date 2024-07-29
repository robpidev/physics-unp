use actix_web::{get, web, HttpResponse, Responder};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/professor").service(get_professors));
}

#[get("")]
async fn get_professors() -> impl Responder {
    HttpResponse::Ok().body("Hello from /professor")
}
