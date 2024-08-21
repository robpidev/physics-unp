use crate::shared;
use serde::Serialize;

mod repository;
pub type DB = shared::repository::db::DB;
pub async fn create(name: &String, faculty_id: &String, db: &DB) -> Result<String, (u16, String)> {
    repository::create(name, faculty_id, db).await
}

pub async fn get(db: &DB) -> Result<impl Serialize, (u16, String)> {
    repository::get(db).await
}

pub async fn delete(id: &String, db: &DB) -> Result<String, (u16, String)> {
    repository::delete(id, db).await
}

pub async fn get_by_id(id: &String, db: &DB) -> Result<impl Serialize, (u16, String)> {
    repository::get_by_id(id, db).await
}
