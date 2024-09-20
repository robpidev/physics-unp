use repository::select_by_course;
use serde::Serialize;

mod repository;

pub async fn students_by_course(course_id: &String) -> Result<impl Serialize, (u16, String)> {
    select_by_course(course_id).await
}
