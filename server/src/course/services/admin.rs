use super::repository::{self, admin};
use serde::Serialize;

pub async fn course(id: &String) -> Result<impl Serialize, (u16, String)> {
    admin::course(id).await
}

pub async fn asign_professor(
    course_id: String,
    professor_id: String,
    role: String,
) -> Result<String, (u16, String)> {
    admin::asign(course_id, professor_id, role).await
}

pub async fn desasign_professor(
    course_id: &String,
    professor_id: &String,
) -> Result<String, (u16, String)> {
    admin::desasign(course_id, professor_id).await
}

pub async fn create(
    name: &String,
    places: u16,
    school_id: &String,
) -> Result<impl Serialize, (u16, String)> {
    repository::create(name, places, school_id).await
}
