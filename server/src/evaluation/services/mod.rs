mod repository;

pub type DB = crate::shared::repository::db::DB;

pub async fn add_evaluation(
    professor_id: &String,
    student_id: &String,
    course_id: &String,
    ev_type: &String,
    score: u8,
    number: u8,
    weight: u8,
    db: &DB,
) -> Result<String, (u16, String)> {
    // TODO: Comprobate that student is enrolled in course

    if !repository::teaches_course(professor_id, course_id, db).await? {
        return Err((400, format!("You aren't practice professor")));
    }

    repository::register_evaluation(professor_id, student_id, ev_type, score, number, weight, db)
        .await
}
