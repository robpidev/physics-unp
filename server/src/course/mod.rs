use actix_web::{
    delete, get,
    http::StatusCode,
    patch, post,
    web::{self},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use serde::Deserialize;

use crate::shared::middlewares::{admin::Admin, student::StudentAuth};

mod services;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/course")
            .service(
                web::scope("/professor")
                    .wrap(Admin)
                    .service(add)
                    .service(delete)
                    //.service(get)
                    .service(get_by_school)
                    .service(register_by_id)
                    .service(unregister)
                    .service(asign)
                    .service(unasign)
                    .service(get_by_professor)
                    .service(get_professors)
                    .service(get)
                    .service(update_test),
            )
            .service(
                web::scope("")
                    .wrap(StudentAuth)
                    //.service(get)
                    .service(get_by_student)
                    .service(get_by_school)
                    .service(register)
                    .service(get_enrolled),
            ),
    );
}

#[derive(Deserialize)]
struct Add {
    name: String,
    places: u16,
    school_id: String,
}

// General

#[get("")]
async fn _get() -> impl Responder {
    match services::get_all().await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("/{id}")]
async fn get(id: web::Path<String>) -> impl Responder {
    match services::get_course(id.to_string()).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[derive(Deserialize)]
struct Test {
    test: u8,
    practice: u8,
}

#[patch("/{id}")]
async fn update_test(id: web::Path<String>, test: web::Form<Test>) -> impl Responder {
    match services::update_test(id.clone(), test.test, test.practice).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

// Professor
#[post("/add")]
async fn add(add: web::Form<Add>) -> impl Responder {
    match services::create(&add.name, add.places, &add.school_id).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[delete("delete/{id}")]
async fn delete(id: web::Path<String>) -> impl Responder {
    match services::delete(&id).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("/courses/{school_id}")]
async fn get_by_school(school_id: web::Path<String>) -> impl Responder {
    match services::get_by_school(&school_id).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("/courses")]
async fn get_by_professor(req: HttpRequest) -> impl Responder {
    let professor_id = req.extensions().get::<String>().unwrap().clone();

    match services::get_by_professor(professor_id).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[post("/register/{course_id}")]
async fn register(course_id: web::Path<String>, req: HttpRequest) -> impl Responder {
    let student_id = match req.extensions().get::<String>() {
        Some(id) => id.clone(),
        None => return HttpResponse::InternalServerError().body("No Student Id"),
    };

    match services::register(&course_id, &student_id).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[derive(Deserialize)]
struct Enroll {
    course_id: String,
    user_id: String,
}

#[post("/register")]
async fn register_by_id(data: web::Form<Enroll>) -> impl Responder {
    match services::register(&data.course_id, &data.user_id).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[delete("/register/")]
async fn unregister(data: web::Form<Enroll>) -> impl Responder {
    match services::unregister(&data.course_id, &data.user_id).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[derive(Deserialize)]
struct Asign {
    course_id: String,
    user_id: String,
    role: String,
}

#[post("/asign")]
async fn asign(data: web::Form<Asign>) -> impl Responder {
    match services::asign_professor(
        data.course_id.clone(),
        data.user_id.clone(),
        data.role.clone(),
    )
    .await
    {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[delete("/asign")]
async fn unasign(data: web::Form<Enroll>) -> impl Responder {
    match services::desasign_professor(&data.course_id, &data.user_id).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}
#[get("/professors/{course_id}")]
async fn get_professors(course_id: web::Path<String>) -> impl Responder {
    match services::get_professors(course_id.to_string()).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

// student
#[get("")]
async fn get_by_student(req: HttpRequest) -> impl Responder {
    let student_id = req.extensions().get::<String>().unwrap().clone();

    match services::get_by_student(&student_id).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("/enrolled")]
async fn get_enrolled(req: HttpRequest) -> impl Responder {
    let student_id = req.extensions().get::<String>().unwrap().clone();
    match services::get_enrolled(&student_id).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}
