use super::repository::enroll;

pub async fn enroll(student_id: String, course_id: String) -> Result<String, (u16, String)> {
    enroll::enroll(student_id, course_id).await
}
