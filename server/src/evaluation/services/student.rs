use serde::Serialize;

use super::repository::student;

///
/// This function return a course evaluations's, name's, and test's
///
pub async fn evaluation(
    student_id: &String,
    course_id: &String,
) -> Result<impl Serialize, (u16, String)> {
    student::evaluation(student_id, course_id).await
}
