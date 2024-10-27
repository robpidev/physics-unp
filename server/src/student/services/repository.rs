use serde::Serialize;

use crate::shared::entities::student::StudentDB;

use crate::shared::repository::db::DB;

pub async fn select_by_course(id_corse: &String) -> Result<impl Serialize, (u16, String)> {
    let query = format!(
        r#"
    SELECT
in.code AS code,
in.names AS names,
in.last_name1 AS last_name1,
in.last_name2 AS last_name2,
in.gender AS gender
FROM course:{}<-enrolled "#,
        id_corse
    );

    let mut resp = match DB.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e))),
    };

    match resp.take::<Vec<StudentDB>>(0) {
        Ok(students) => Ok(students),
        Err(e) => return Err((500, format!("DB parse error: {}", e))),
    }
}

// TODO: Return student only becomes a the school
pub async fn info(student_id: String) -> Result<impl Serialize, (u16, String)> {
    let query = r#"SELECT * omit password FROM type::thing('student', <int>$student_id)"#;

    let mut resp = match DB.query(query).bind(("student_id", student_id)).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    let student: Option<StudentDB> = match resp.take(0) {
        Ok(s) => s,
        Err(e) => return Err((500, format!("DB parse error: {}", e.to_string()))),
    };

    match student {
        Some(s) => Ok(s),
        None => Err((404, "Student not found".to_string())),
    }
}
