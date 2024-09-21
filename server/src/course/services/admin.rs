use super::repository::admin;
use serde::Serialize;

pub async fn course(id: &String) -> Result<impl Serialize, (u16, String)> {
    admin::course(id).await
}
