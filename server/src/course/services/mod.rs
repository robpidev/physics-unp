use serde::Serialize;

use crate::shared;

mod repository;

pub type DB = shared::repository::db::DB;

pub async fn create(name: &String, db: &DB) -> Result<impl Serialize, (u16, String)> {
    repository::create(name, db).await
}

pub async fn delete(id: &String, db: &DB) -> Result<String, (u16, String)> {
    repository::delete(id, db).await
}

pub async fn get_all(db: &DB) -> Result<impl Serialize, (u16, String)> {
    repository::get_all(db).await
}
