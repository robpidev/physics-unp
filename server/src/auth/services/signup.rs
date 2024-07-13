use crate::auth::{entities::professor::Professor, repository::signup};

pub type DB = crate::shared::repository::db::DB;

pub async fn signup_professor(
    names: String,
    last_name1: String,
    last_name2: String,
    dni: String,
    password: String,
    db: &DB,
) -> Result<String, (u16, String)> {
    let professor = match Professor::new(names, last_name1, last_name2, dni, password) {
        Ok(p) => p,
        Err(e) => return Err((400u16, e)),
    };

    signup::create_professor(professor, db).await
}
