use serde::Serialize;

mod repository;

pub async fn create(
    name: &String,
    places: u16,
    school_id: &String,
) -> Result<impl Serialize, (u16, String)> {
    repository::create(name, places, school_id).await
}

pub async fn delete(id: &String) -> Result<String, (u16, String)> {
    repository::delete(id).await
}

pub async fn get_all() -> Result<impl Serialize, (u16, String)> {
    repository::get_all().await
}

pub async fn get_by_school(id: &String) -> Result<impl Serialize, (u16, String)> {
    repository::get_by_school(id).await
}

pub async fn get_by_professor(professor_id: String) -> Result<impl Serialize, (u16, String)> {
    repository::get_by_professor(professor_id).await
}

pub async fn register(course_id: &String, student_id: &String) -> Result<String, (u16, String)> {
    if !repository::check_id(course_id).await? {
        return Err((400, format!("Course dont exists: {}", course_id)));
    }

    repository::enroll(student_id, course_id).await
}

pub async fn unregister(course_id: &String, student_id: &String) -> Result<String, (u16, String)> {
    repository::unregister(student_id, course_id).await
}

pub async fn asign_professor(
    course_id: &String,
    professor_id: &String,
    role: &String,
) -> Result<String, (u16, String)> {
    repository::asign_professor(course_id, professor_id, role).await
}

pub async fn desasign_professor(
    course_id: &String,
    professor_id: &String,
) -> Result<String, (u16, String)> {
    repository::desasign_professor(course_id, professor_id).await
}

pub async fn get_by_student(student_id: &String) -> Result<impl Serialize, (u16, String)> {
    repository::get_by_student(student_id).await
}

pub async fn get_enrolled(student_id: &String) -> Result<impl Serialize, (u16, String)> {
    repository::get_enrolled(student_id).await
}

pub async fn get_professors(course_id: String) -> Result<impl Serialize, (u16, String)> {
    repository::get_professors(course_id).await
}

pub async fn get_course(course_id: String) -> Result<impl Serialize, (u16, String)> {
    repository::get_course(course_id).await
}

pub async fn update_test(
    course_id: String,
    test: u8,
    weight: u8,
) -> Result<impl Serialize, (u16, String)> {
    repository::update_test(course_id, test, weight).await
}
