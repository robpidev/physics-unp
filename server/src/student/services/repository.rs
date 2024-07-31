use serde::Serialize;

use crate::shared::entities::student::StudentDB;

use super::DB;

pub async fn select_by_course(id_corse: &String, db: &DB) -> Result<impl Serialize, (u16, String)> {
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

    let mut resp = match db.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e))),
    };

    match resp.take::<Vec<StudentDB>>(0) {
        Ok(students) => Ok(students),
        Err(e) => return Err((500, format!("DB parse error: {}", e))),
    }
}
