use serde::Serialize;

mod repository;
pub mod student;

pub async fn add_evaluation(
    professor_id: String,
    student_id: String,
    course_id: String,
    ev_type: String,
    score: f32,
    number: u8,
) -> Result<String, (u16, String)> {
    // TODO: Comprobate that student is enrolled in course

    if !repository::teaches_course(professor_id, course_id.clone()).await? {
        return Err((400, format!("You aren't practice professor")));
    }

    repository::register_evaluation(course_id, student_id, ev_type, score, number).await
}

pub async fn update_evaluation(
    ev_id: String,
    score: f32,
    number: u8,
    professor_id: String,
    course_id: String,
) -> Result<String, (u16, String)> {
    if !repository::teaches_course(professor_id, course_id).await? {
        return Err((400, format!("You aren't practice professor")));
    }

    repository::update_evaluation(ev_id, score, number).await
}

pub async fn get_evaluations(
    student_id: &String,
    course_id: &String,
) -> Result<impl Serialize, (u16, String)> {
    repository::get_evaluation(student_id, course_id).await
}

/// This function returns all the evaluations of a course:
/// ```
/// evaluations: [
///     {
///         name: studet_name,
///         id: student_id,
///         scores: [
///          id: score_id,
///          number: practice_number,
///          score: evaluation_puntuation,
///          ev_type: type_evaluation
///         ]
///     }
/// ]
/// ```
pub async fn get_all_evaluations(course_id: String) -> Result<impl Serialize, (u16, String)> {
    repository::get_all_evaluations_course(course_id).await
}
