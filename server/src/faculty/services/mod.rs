mod repository;

use serde::Serialize;

pub async fn create(name: &String) -> Result<String, (u16, String)> {
    repository::create(name).await
}

pub async fn get() -> Result<impl Serialize, (u16, String)> {
    repository::get().await
}

pub async fn delete(name: &String) -> Result<String, (u16, String)> {
    repository::delete(name).await
}
