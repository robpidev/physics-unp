use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::DB;

// TODO: Solve problem whit faculty_id this create the relation for any faculty_id
pub async fn create(name: &String, faculty_id: &String, db: &DB) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
(RELATE faculty:{}
-> includes ->
(CREATE school SET name = '{}')).out.name;
    "#,
        faculty_id, name
    );

    let mut resp = match db.query(query).await {
        Ok(resp) => resp,
        Err(err) => return Err((500, format!("Database error: {}", err.to_string()))),
    };

    let name = match resp.take::<Option<String>>(0) {
        Ok(school) => school,
        Err(err) => return Err((500, format!("DB Error, School Exists: {}", err.to_string()))),
    };

    match name {
        Some(name) => Ok(name),
        None => Err((500, "Database error".to_string())),
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

    let mut resp = match db.query(&query).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB conection error: {}", e.to_string()))),
    };

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

pub async fn delete(db: &DB) -> Result<String, (u16, String)> {
    let query = format!(r#"DELETE school;"#);
    todo!();
}
