use crate::auth::repository::signin;

pub type DB = crate::shared::repository::db::DB;

pub async fn login(id: String, password: String, db: &DB) -> Result<String, (u16, String)> {
    if id.len() == 8 {
        signin::sign_in(id, password, "professor", &db).await
    } else {
        signin::sign_in(id, password, "student", &db).await
    }
}
