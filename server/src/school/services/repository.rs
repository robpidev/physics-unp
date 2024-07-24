use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Response};

use super::DB;

// TODO: Solve problem whit faculty_id this create the relation for any faculty_id
pub async fn create(name: &String, faculty_id: &String, db: &DB) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
IF (SELECT * FROM {}) != [] THEN
	(RELATE {} -> includes -> (CREATE school SET name = '{}')).out.name
ELSE
	(RETURN NONE)
END;
    "#,
        faculty_id, faculty_id, name
    );

    let mut resp = make_petition(&query, db).await?;
    let name = match resp.take::<Option<String>>(0) {
        Ok(school) => school,
        Err(_) => return Err((500, format!("DB Error, School Exists"))),
    };

    match name {
        Some(name) => Ok(format!("School created: {}", name)),
        None => Err((401, "Faculty id dont't exist".to_string())),
    }
}

#[derive(Deserialize, Serialize)]
struct SchoolDB {
    id: Thing,
    name: String,
}

#[derive(Deserialize, Serialize)]
struct School {
    id: String,
    name: String,
}

pub async fn get(db: &DB) -> Result<impl Serialize, (u16, String)> {
    let query = format!(r#"SELECT id, name FROM school;"#);

    let mut resp = make_petition(&query, db).await?;

    match resp.take::<Vec<SchoolDB>>(0) {
        Ok(schools) => Ok(schools
            .into_iter()
            .map(|s| School {
                id: s.id.to_string(),
                name: s.name,
            })
            .collect::<Vec<School>>()),
        Err(e) => return Err((400, format!("DB search error: {}", e.to_string()))),
    }
}

pub async fn delete(id: &String, db: &DB) -> Result<String, (u16, String)> {
    let query = format!(r#"DELETE {} RETURN BEFORE;"#, id);

    let mut result = make_petition(&query, db).await?;

    let school = match result.take::<Option<SchoolDB>>(0) {
        Ok(school) => school,
        Err(e) => return Err((500, format!("DB error: {}", e.to_string()))),
    };
    match school {
        Some(school) => Ok(format!("School deleted: {}", school.name)),
        None => return Err((401, "School don't exist".to_string())),
    }
}

async fn make_petition(query: &String, db: &DB) -> Result<Response, (u16, String)> {
    match db.query(query).await {
        Ok(resp) => Ok(resp),
        Err(e) => Err((500, format!("DB query error: {}", e.to_string()))),
    }
}
