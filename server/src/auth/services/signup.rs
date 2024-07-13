use crate::auth::{
    entities::{professor::Professor, student::Student},
    repository::signup,
};

pub type DB = crate::shared::repository::db::DB;

pub async fn professor(
    names: String,
    last_name1: String,
    last_name2: String,
    dni: String,
    password: String,
    gender: bool,
    db: &DB,
) -> Result<String, (u16, String)> {
    let professor = match Professor::new(names, last_name1, last_name2, dni, password, gender) {
        Ok(p) => p,
        Err(e) => return Err((400u16, e)),
    };

    signup::register_professor(professor, db).await
}

pub async fn student(
    code: String,
    names: String,
    last_name1: String,
    last_name2: String,
    password: String,
    gender: bool,
    db: &DB,
) -> Result<String, (u16, String)> {
    let student = match Student::new(code, names, last_name1, last_name2, password, gender) {
        Ok(s) => s,
        Err(e) => return Err((400u16, e)),
    };

    signup::register_studend(student, db).await
}
