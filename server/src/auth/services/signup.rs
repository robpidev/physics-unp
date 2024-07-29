use super::{
    entities::{professor::Professor, student::Student},
    repository::signup::{self, verify_school},
};

pub type DB = crate::shared::repository::db::DB;

pub async fn register(
    id: String,
    password: String,
    user_type: String,
    names: String,
    last_name1: String,
    last_name2: String,
    gender: bool,
    school_id: &String,
    db: &DB,
) -> Result<String, (u16, String)> {
    if user_type == "professor" {
        match Professor::new(id.clone(), names, last_name1, last_name2, password, gender) {
            Ok(p) => return signup::save(p, id, user_type, school_id, &db).await,
            Err(e) => return Err((400u16, e)),
        };
    } else if user_type == "student" {
        verify_school(school_id, &db).await?;

        match Student::new(id.clone(), names, last_name1, last_name2, password, gender) {
            Ok(p) => return signup::save(p, id, user_type, school_id, &db).await,
            Err(e) => return Err((400u16, e)),
        };
    } else {
        return Err((400u16, "Invalid user type".to_string()));
    };
}
