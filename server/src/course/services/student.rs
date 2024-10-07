use serde::Serialize;

use super::repository::student;

pub async fn enrolled(student_id: String) -> Result<impl Serialize, (u16, String)> {
    student::enrolled(student_id).await
}

pub async fn avilables(student_id: String) -> Result<impl Serialize, (u16, String)> {
    student::avilables(student_id).await
}

pub async fn enroll(
    student_id: String,
    course_id: String,
    ocupated_groups: Vec<u8>,
) -> Result<String, (u16, String)> {
    student::enroll(student_id, course_id, ocupated_groups).await
}
