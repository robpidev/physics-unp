use crate::shared::repository::db::DB;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub mod admin;
pub mod enroll;
pub mod professor;
pub mod student;
pub mod test;

#[derive(Serialize, Deserialize)]
struct CourseDB {
    id: Thing,
    places: u16,
    name: String,
    enrolled: u16,
}

#[derive(Serialize)]
struct Course {
    id: String,
    name: String,
    places: u16,
    enrolled: u16,
}

impl CourseDB {
    fn map(self) -> Course {
        Course {
            id: self.id.id.to_string(),
            name: self.name,
            places: self.places,
            enrolled: self.enrolled,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CourseTest {
    name: String,
    places: u16,
    tests: Vec<Test>,
}

#[derive(Serialize, Deserialize)]
struct Test {
    name: String,
    weight: u8,
}

pub async fn create(
    name: &String,
    places: u16,
    school_id: &String,
) -> Result<impl Serialize, (u16, String)> {
    let query = format!(
        r#"
BEGIN TRANSACTION;
IF (SELECT * FROM school:{}) != [] THEN
	(SELECT out.id AS id, out.name AS name, out.places AS places, 0 AS enrolled FROM
  (RELATE school:{} -> offers -> (CREATE course CONTENT {{
		name: "{}",
		places: {},
    tests: [{{name: "test", weight: 50}}, {{name: "práctice", weight: 50}}],
	}})))
ELSE
	(RETURN NONE)
END;
COMMIT TRANSACTION;
    "#,
        school_id, school_id, name, places
    );

    let mut resp = match DB.query(query).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    let course = match resp.take::<Option<CourseDB>>(0) {
        Ok(c) => c,
        Err(e) => return Err((400, format!("Course exists: {}", e.to_string()))),
    };

    match course {
        Some(c) => Ok(c.map()),
        None => Err((400, format!("School id don't exist: {}", name))),
    }
}

#[derive(Deserialize)]
struct CourseDeleted {
    name: String,
}

pub async fn delete(id: &String) -> Result<String, (u16, String)> {
    let query = format!("DELETE course:{}  RETURN BEFORE;", id);

    let mut resp = match DB.query(query).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB id Error: {}", e.to_string()))),
    };

    let course = match resp.take::<Option<CourseDeleted>>(0) {
        Ok(c) => c,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match course {
        Some(c) => Ok(format!("Course {} deleted", c.name)),
        None => Err((400, format!("Course dont exists: {}", id))),
    }
}

pub async fn get_all() -> Result<impl Serialize, (u16, String)> {
    let mut resp = match DB.query("SELECT * FROM course;").await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Vec<CourseDB>>(0) {
        Ok(resp) => Ok(resp.into_iter().map(|c| c.map()).collect::<Vec<Course>>()),
        Err(e) => Err((500, format!("DB Error: {}", e.to_string()))),
    }
}

pub async fn get_by_school(school_id: &String) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
SELECT name, places, id, count(<-enrolled) AS enrolled
FROM type::thing("school", $school_id)->offers->course;
    "#;

    let mut resp = match DB.query(query).bind(("school_id", school_id.clone())).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Vec<CourseDB>>(0) {
        Ok(resp) => Ok(resp.into_iter().map(|c| c.map()).collect::<Vec<Course>>()),
        Err(e) => Err((500, format!("DB Error: {}", e.to_string()))),
    }
}

pub async fn exists(id: &String) -> Result<bool, (u16, String)> {
    let query = format!("SELECT * FROM ONLY course:{};", id);

    let mut resp = match DB.query(query).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Option<CourseDB>>(0) {
        Ok(c) => match c {
            Some(_) => Ok(true),
            None => Ok(false),
        },
        Err(e) => Err((500, format!("Incorrect course ID: {}", e.to_string()))),
    }
}

pub async fn update_places(id: String, places: u16) -> Result<String, (u16, String)> {
    let query = r#"update type::thing("course", $id) set places=<int>$places"#;

    let resp = match DB
        .query(query)
        .bind(("places", places))
        .bind(("id", id))
        .await
    {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.check() {
        Ok(_) => Ok(format!("Course updated")),
        Err(e) => Err((500, format!("DB Error: {}", e.to_string()))),
    }
}
