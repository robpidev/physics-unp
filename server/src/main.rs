use std::env;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use server::{auth, course, faculty, professor, school};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load db
    dotenv().ok();
    let db_host = env::var("DB_HOST").unwrap();
    let db = server::shared::repository::db::db_connect(db_host)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(hello)
            .configure(auth::routes)
            .configure(faculty::routes)
            .configure(school::routes)
            .configure(course::routes)
            .configure(professor::routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
