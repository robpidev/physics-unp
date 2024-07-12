use actix_web::{get, web};
// pub mod signin;
pub mod signup;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(auth).configure(signup::config));
}

#[get("")]
async fn auth() -> String {
    format!("Hello from auth")
}
