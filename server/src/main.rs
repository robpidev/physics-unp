use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use server::{auth, faculty};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = server::shared::repository::db::db_connect().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(hello)
            .configure(auth::routes::config)
            .configure(faculty::routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
