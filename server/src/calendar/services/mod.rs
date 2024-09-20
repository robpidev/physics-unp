use serde::Serialize;

mod repository;

pub async fn get_datetimes() -> Result<impl Serialize, (u16, String)> {
    repository::get_datetimes().await
}

pub async fn delete(id: String) -> Result<(), (u16, String)> {
    repository::delete(id).await
}

pub async fn add(todo: String, to: String) -> Result<impl Serialize, (u16, String)> {
    repository::add(todo, to).await
}
