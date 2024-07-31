mod repository;
use serde::Serialize;

pub type DB = crate::shared::repository::db::DB;

pub async fn get_all(db: &DB) -> Result<impl Serialize, (u16, String)> {
    repository::get_all(db).await
}
