use actix_web::web;

mod services;

mod professor;
mod student;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/evaluation")
            .configure(student::routes)
            .configure(professor::routes), //.wrap(Professor)
    );
}
