use crate::auth::{
    entities::{professor::Professor, student::Student},
    repository::signup,
};

pub type DB = crate::shared::repository::db::DB;

pub async fn register(
    names: String,
    last_name1: String,
    last_name2: String,
    dni: String,
    password: String,
    gender: bool,
    user_type: String,
    db: &DB,
) -> Result<String, (u16, String)> {
    if user_type == "professor" {
        match Professor::new(names, last_name1, last_name2, dni.clone(), password, gender) {
            Ok(p) => return signup::save(p, dni, user_type, &db).await,
            Err(e) => return Err((400u16, e)),
        };
    } else if user_type == "student" {
        match Student::new(names, last_name1, last_name2, dni.clone(), password, gender) {
            Ok(p) => return signup::save(p, dni, user_type, &db).await,
            Err(e) => return Err((400u16, e)),
        };
    } else {
        return Err((400u16, "Invalid user type".to_string()));
    };
}
