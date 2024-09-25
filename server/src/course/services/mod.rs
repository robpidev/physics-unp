use serde::Serialize;

pub mod admin;
pub mod professor;
pub mod student;

mod repository;

pub async fn delete(id: &String) -> Result<String, (u16, String)> {
    repository::delete(id).await
}

pub async fn get_all() -> Result<impl Serialize, (u16, String)> {
    repository::get_all().await
}

pub async fn get_by_school(id: &String) -> Result<impl Serialize, (u16, String)> {
    repository::get_by_school(id).await
}

pub async fn register(course_id: &String, student_id: &String) -> Result<String, (u16, String)> {
    if !repository::exists(course_id).await? {
        return Err((400, format!("Course dont exists: {}", course_id)));
    }

    repository::enroll::new(student_id, course_id).await
}

pub async fn unregister(course_id: &String, student_id: &String) -> Result<String, (u16, String)> {
    repository::enroll::unregister(student_id, course_id).await
}

pub async fn get_by_student(student_id: &String) -> Result<impl Serialize, (u16, String)> {
    repository::student::enrolled(student_id.to_string()).await
}

pub async fn update_test(
    course_id: String,
    test: u8,
    weight: u8,
) -> Result<impl Serialize, (u16, String)> {
    repository::test::update(course_id, test, weight).await
}

pub async fn update_places(id: String, places: u16) -> Result<String, (u16, String)> {
    repository::update_places(id, places).await
}
