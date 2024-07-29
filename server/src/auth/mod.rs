mod services;

use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web, HttpResponse, Responder,
};

// pub mod signin;
mod signin;
mod signup;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(auth)
            .configure(signup::config)
            .configure(signin::config),
    );
}

#[get("")]
async fn auth() -> String {
    format!("Hello from auth")
}

pub fn response(data: Result<String, (u16, String)>) -> impl Responder {
    match data {
        Ok(s) => HttpResponse::Ok().content_type(ContentType::json()).body(s),

        Err((code, message)) => {
            HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(message)
        }
    }
}
