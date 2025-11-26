use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use serde::Deserialize;

mod services;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notices")
            .service(
                web::scope("/admin")
                    .wrap(crate::shared::middlewares::admin::Admin)
                    .service(create_notice),
            )
            .service(get_notices),
    );
}

#[get("")]
async fn get_notices() -> impl Responder {
    "Hello from notices"
}

#[derive(Deserialize)]
struct Notice {
    note: String,
}

#[post("")]
async fn create_notice(data: web::Form<Notice>) -> impl Responder {
    match services::NoticeService::create(data.note.clone()).await {
        Ok(n) => HttpResponse::Ok().json(n),
        Err((c, m)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(m),
    }
}
