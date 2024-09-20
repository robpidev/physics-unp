use serde::Serialize;

mod repository;
pub async fn create(name: &String, faculty_id: &String) -> Result<String, (u16, String)> {
    repository::create(name, faculty_id).await
}

pub async fn get() -> Result<impl Serialize, (u16, String)> {
    repository::get().await
}

pub async fn delete(id: &String) -> Result<String, (u16, String)> {
    repository::delete(id).await
}

pub async fn get_by_id(id: String) -> Result<impl Serialize, (u16, String)> {
    repository::get_by_id(id).await
}
