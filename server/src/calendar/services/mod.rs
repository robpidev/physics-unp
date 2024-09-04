use serde::Serialize;

mod repository;
pub type DB = repository::DB;

pub async fn get_datetimes(db: &DB) -> Result<impl Serialize, (u16, String)> {
    repository::get_datetimes(db).await
}
