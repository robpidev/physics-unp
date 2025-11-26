use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use server::{auth, calendar, config, course, evaluation, faculty, professor, school, student};

use server::config::Server;
use server::shared::repository::DBInstance;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is starting...");

    // Load .env file
    config::load_env();

    // Config server
    let server_config = Server::from_env();

    println!("Host: {}", server_config.host);
    println!("Port: {}", server_config.port);

    //  DB Instance
    if let Err(e) = DBInstance::db_connect().await {
        panic!("Error connecting to DB: {}", e);
    }

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
    .bind((server_config.host, server_config.port))
    .inspect(|_| println!("\x1b[32mServer is running"))?
    .run()
    .await
}
