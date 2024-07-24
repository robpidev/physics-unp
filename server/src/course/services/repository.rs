use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::shared::entities::course::Course;

use super::DB;

#[derive(Serialize, Deserialize)]
struct CourseDB {
    id: Thing,
    name: String,
}

pub async fn create(name: &String, db: &DB) -> Result<impl Serialize, (u16, String)> {
    let query = format!(r#"CREATE course set name = "{}";"#, name);

    let mut resp = match db.query(query).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    let course = match resp.take::<Option<CourseDB>>(0) {
        Ok(c) => c,
        Err(e) => return Err((400, format!("Course exists: {}", e.to_string()))),
    };

    match course {
        Some(c) => Ok(Course {
            id: c.id.to_string(),
            name: c.name,
        }),
        None => Err((400, format!("Course not found: {}", name))),
    }
}

pub async fn delete(id: &String, db: &DB) -> Result<String, (u16, String)> {
    let query = format!("DELETE {}  RETURN BEFORE;", id);

    let mut resp = match db.query(query).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB id Error: {}", e.to_string()))),
    };

    let course = match resp.take::<Option<CourseDB>>(0) {
        Ok(c) => c,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match course {
        Some(c) => Ok(format!("Course {} deleted", c.name)),
        None => Err((400, format!("Course dont exists: {}", id))),
    }
}

pub async fn get_all(db: &DB) -> Result<impl Serialize, (u16, String)> {
    let mut resp = match db.query("SELECT * FROM course;").await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Vec<CourseDB>>(0) {
        Ok(resp) => Ok(resp
            .into_iter()
            .map(|c| Course {
                id: c.id.to_string(),
                name: c.name,
            })
            .collect::<Vec<Course>>()),
        Err(e) => Err((500, format!("DB Error: {}", e.to_string()))),
    }
}
