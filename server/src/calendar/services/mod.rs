use serde::Serialize;

mod repository;
pub type DB = repository::DB;

pub async fn get_datetimes(db: &DB) -> Result<impl Serialize, (u16, String)> {
    repository::get_datetimes(db).await
}

pub async fn delete(id: &String, db: &DB) -> Result<(), (u16, String)> {
    repository::delete(id, db).await
}

pub async fn add(todo: &String, to: &String, db: &DB) -> Result<impl Serialize, (u16, String)> {
    repository::add(todo, to, db).await
}
