use super::repository::professor;
use serde::Serialize;
pub async fn courses(professor_id: String) -> Result<impl Serialize, (u16, String)> {
    professor::courses(professor_id).await
}

pub async fn info(course_id: String) -> Result<impl Serialize, (u16, String)> {
    professor::info(course_id).await
}
