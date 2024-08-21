mod repository;

pub type DB = crate::shared::repository::db::DB;

use serde::Serialize;

pub async fn create(name: &String, db: &DB) -> Result<String, (u16, String)> {
    repository::create(name, db).await
}

pub async fn get(db: &DB) -> Result<impl Serialize, (u16, String)> {
    repository::get(db).await
}

pub async fn delete(name: &String, db: &DB) -> Result<String, (u16, String)> {
    repository::delete(name, db).await
}
