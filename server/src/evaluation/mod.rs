use actix_web::{
    get,
    http::StatusCode,
    patch, post,
    web::{self},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use serde::Deserialize;
use services::{add_evaluation, DB};

use crate::shared::middlewares::admin::Admin;

mod services;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/evaluation")
            .wrap(Admin)
            .service(get)
            .service(get_all)
            .service(add)
            .service(add_with_id)
            .service(update),
    );
}

#[derive(Deserialize)]
struct Data {
    student_id: String,
    course_id: String,
}

#[get("")]
async fn get(data: web::Form<Data>, db: web::Data<DB>) -> impl Responder {
    match services::get_evaluations(&data.student_id, &data.course_id, &db).await {
        Ok(evs) => HttpResponse::Ok().json(evs),
        Err((c, e)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(e),
    }
}

#[derive(Deserialize)]
struct Evaluation {
    course_id: String,
    student_id: String,
    evaluation_type: String,
    score: f32,
    number: u8,
    weight: u8,
}

#[post("")]
async fn add(data: web::Form<Evaluation>, req: HttpRequest, db: web::Data<DB>) -> impl Responder {
    let professor_id = req.extensions().get::<String>().unwrap().clone();

    match add_evaluation(
        &professor_id,
        &data.student_id,
        &data.course_id,
        &data.evaluation_type,
        data.score,
        data.number,
        data.weight,
        &db,
    )
    .await
    {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((c, e)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(e),
    }
}

#[derive(Deserialize)]
struct EvaluationUpdate {
    ev_id: String,
    score: f32,
    weight: u8,
    number: u8,
    course_id: String,
}

#[patch("")]
async fn update(
    data: web::Form<EvaluationUpdate>,
    req: HttpRequest,
    db: web::Data<DB>,
) -> impl Responder {
    let professor_id = req.extensions().get::<String>().unwrap().clone();
    match services::update_evaluation(
        &data.ev_id,
        data.score,
        data.weight,
        data.number,
        &professor_id,
        &data.course_id,
        &db,
    )
    .await
    {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((c, e)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(e),
    }
}

#[derive(Deserialize)]
struct EvaluationID {
    professor_id: String,
    course_id: String,
    student_id: String,
    evaluation_type: String,
    score: f32,
    number: u8,
    weight: u8,
}

#[post("/add")]
async fn add_with_id(data: web::Form<EvaluationID>, db: web::Data<DB>) -> impl Responder {
    match add_evaluation(
        &data.professor_id,
        &data.student_id,
        &data.course_id,
        &data.evaluation_type,
        data.score,
        data.number,
        data.weight,
        &db,
    )
    .await
    {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err((c, e)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(e),
    }
}

#[get("/all/{course_id}")]
async fn get_all(course_id: web::Path<String>, db: web::Data<DB>) -> impl Responder {
    match services::get_all_evaluations(&course_id, &db).await {
        Ok(evs) => HttpResponse::Ok().json(evs),
        Err((c, e)) => HttpResponse::build(StatusCode::from_u16(c).unwrap()).body(e),
    }
}
