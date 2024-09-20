mod repository;
use serde::Serialize;

pub async fn get_all() -> Result<impl Serialize, (u16, String)> {
    repository::get_all().await
}
