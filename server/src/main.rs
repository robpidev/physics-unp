use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use server::{auth, calendar, course, evaluation, faculty, professor, school, student};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load db
    let config = server::app_config();

    server::shared::repository::db::db_connect(
        config.db_host,
        config.db_user,
        config.db_pass,
        config.db_use_ns,
        config.db_use_db,
    )
    .await
    .unwrap();

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .configure(auth::routes)
            .configure(faculty::routes)
            .configure(school::routes)
            .configure(course::routes)
            .configure(professor::routes)
            .configure(evaluation::routes)
            .configure(student::routes)
            .configure(calendar::routes)
    })
    .bind(config.host)?
    .run()
    .await
}
