use std::env;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use server::{auth, calendar, course, evaluation, faculty, professor, school, student};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load db
    dotenv().ok();
    let db_host = env::var("DB_HOST").unwrap();
    let host = env::var("HOST").unwrap();

    server::shared::repository::db::db_connect(db_host)
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
    .bind(host)?
    .run()
    .await
}
