use serde::Serialize;

use super::repository::signin;

pub type DB = crate::shared::repository::db::DB;

pub async fn login(id: String, password: String, db: &DB) -> Result<impl Serialize, (u16, String)> {
    if id.len() == 8 {
        signin::sign_in(id, password, "professor", &db).await
    } else if id.len() == 10 {
        signin::sign_in(id, password, "student", &db).await
    } else {
        Err((400, "Code invalid".to_string()))
    }
}
