use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use server::auth;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = server::shared::repository::db::db_connect().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(hello)
            .configure(auth::routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
