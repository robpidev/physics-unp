use actix_web::{get, post, web};

use crate::shared::middlewares::admin::Admin;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/faculty").wrap(Admin).service(faculty));
}

#[get("")]
async fn faculty() -> String {
    format!("hello from faculty")
}

#[post("/add")]
async fn add(name: web::Form<String>) -> String {
    format!("hello {} from add", name)
}
