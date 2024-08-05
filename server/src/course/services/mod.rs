use serde::Serialize;

use crate::shared;

mod repository;

pub type DB = shared::repository::db::DB;

pub async fn create(
    name: &String,
    places: u16,
    school_id: &String,
    db: &DB,
) -> Result<impl Serialize, (u16, String)> {
    repository::create(name, places, school_id, db).await
}

pub async fn delete(id: &String, db: &DB) -> Result<String, (u16, String)> {
    repository::delete(id, db).await
}

pub async fn get_all(db: &DB) -> Result<impl Serialize, (u16, String)> {
    repository::get_all(db).await
}

pub async fn get_by_school(id: &String, db: &DB) -> Result<impl Serialize, (u16, String)> {
    repository::get_by_school(id, db).await
}

pub async fn get_by_professor(
    professor_id: &String,
    db: &DB,
) -> Result<impl Serialize, (u16, String)> {
    repository::get_by_professor(professor_id, db).await
}

pub async fn register(
    course_id: &String,
    student_id: &String,
    db: &DB,
) -> Result<String, (u16, String)> {
    if !repository::check_id(course_id, db).await? {
        return Err((400, format!("Course dont exists: {}", course_id)));
    }

    repository::enroll(student_id, course_id, db).await
}

pub async fn unregister(
    course_id: &String,
    student_id: &String,
    db: &DB,
) -> Result<String, (u16, String)> {
    repository::unregister(student_id, course_id, db).await
}

pub async fn asign_professor(
    course_id: &String,
    professor_id: &String,
    role: &String,
    db: &DB,
) -> Result<String, (u16, String)> {
    repository::asign_professor(course_id, professor_id, role, db).await
}

pub async fn desasign_professor(
    course_id: &String,
    professor_id: &String,
    db: &DB,
) -> Result<String, (u16, String)> {
    repository::desasign_professor(course_id, professor_id, db).await
}

pub async fn get_by_student(student_id: &String, db: &DB) -> Result<impl Serialize, (u16, String)> {
    repository::get_by_student(student_id, db).await
}

pub async fn get_enrolled(student_id: &String, db: &DB) -> Result<impl Serialize, (u16, String)> {
    repository::get_enrolled(student_id, db).await
}
