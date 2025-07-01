use super::repository::professor;
use serde::Serialize;
pub async fn courses(professor_id: String) -> Result<impl Serialize, (u16, String)> {
    professor::courses(professor_id).await
}

pub async fn course_info(
    course_id: String,
    professor_id: String,
) -> Result<impl Serialize, (u16, String)> {
    professor::course_info(course_id, professor_id).await
}
