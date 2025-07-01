use serde::Serialize;

use super::repository::student;

pub async fn enrolled(student_id: String) -> Result<impl Serialize, (u16, String)> {
    student::enrolled(student_id).await
}

pub async fn avilables(student_id: String) -> Result<impl Serialize, (u16, String)> {
    student::avilables(student_id).await
}
