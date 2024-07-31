use repository::select_by_course;
use serde::Serialize;

mod repository;

pub type DB = crate::shared::repository::db::DB;

pub async fn students_by_course(
    course_id: &String,
    db: &DB,
) -> Result<impl Serialize, (u16, String)> {
    select_by_course(course_id, db).await
}
