use actix_web::{
    delete, get, http::StatusCode, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
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
                    .service(get_by_professor),
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
async fn _get(db: web::Data<services::DB>) -> impl Responder {
    match services::get_all(&db).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

// Professor
#[post("/add")]
async fn add(add: web::Form<Add>, db: web::Data<services::DB>) -> impl Responder {
    match services::create(&add.name, add.places, &add.school_id, &db).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[delete("delete/{id}")]
async fn delete(id: web::Path<String>, db: web::Data<services::DB>) -> impl Responder {
    match services::delete(&id, &db).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("/courses/{school_id}")]
async fn get_by_school(
    school_id: web::Path<String>,
    db: web::Data<services::DB>,
) -> impl Responder {
    match services::get_by_school(&school_id, &db).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("/courses")]
async fn get_by_professor(db: web::Data<services::DB>, req: HttpRequest) -> impl Responder {
    let professor_id = req.extensions().get::<String>().unwrap().clone();

    match services::get_by_professor(&professor_id, &db).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[post("/register/{course_id}")]
async fn register(
    course_id: web::Path<String>,
    req: HttpRequest,
    db: web::Data<services::DB>,
) -> impl Responder {
    let student_id = match req.extensions().get::<String>() {
        Some(id) => id.clone(),
        None => return HttpResponse::InternalServerError().body("No Student Id"),
    };

    match services::register(&course_id, &student_id, &db).await {
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
async fn register_by_id(data: web::Form<Enroll>, db: web::Data<services::DB>) -> impl Responder {
    match services::register(&data.course_id, &data.user_id, &db).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[delete("/register/")]
async fn unregister(data: web::Form<Enroll>, db: web::Data<services::DB>) -> impl Responder {
    match services::unregister(&data.course_id, &data.user_id, &db).await {
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
async fn asign(data: web::Form<Asign>, db: web::Data<services::DB>) -> impl Responder {
    match services::asign_professor(&data.course_id, &data.user_id, &data.role, &db).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[delete("/asign")]
async fn unasign(data: web::Form<Enroll>, db: web::Data<services::DB>) -> impl Responder {
    match services::desasign_professor(&data.course_id, &data.user_id, &db).await {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

// student
#[get("")]
async fn get_by_student(db: web::Data<services::DB>, req: HttpRequest) -> impl Responder {
    let student_id = req.extensions().get::<String>().unwrap().clone();

    match services::get_by_student(&student_id, &db).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}

#[get("/enrolled")]
async fn get_enrolled(db: web::Data<services::DB>, req: HttpRequest) -> impl Responder {
    let student_id = req.extensions().get::<String>().unwrap().clone();
    match services::get_enrolled(&student_id, &db).await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err((code, msg)) => HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(msg),
    }
}
