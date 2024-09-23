use crate::shared::repository::db::DB;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Deserialize)]
struct CourseDB {
    role: String,
    name: String,
    school: String,
    id: Thing,
}

#[derive(Serialize)]
struct Course {
    role: String,
    name: String,
    school: String,
    id: String,
}

impl CourseDB {
    fn map(self) -> Course {
        Course {
            role: self.role,
            name: self.name,
            school: self.school,
            id: self.id.id.to_string(),
        }
    }
}

pub async fn courses(professor_id: String) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
SELECT role, out.name AS name, out.id AS id,
(->course<-offers<-school.name)[0] as school 
FROM type::thing("professor", <int>$professor_id)->teaches;
    "#;

    let mut resp = match DB.query(query).bind(("professor_id", professor_id)).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Vec<CourseDB>>(0) {
        Ok(courses) => Ok(courses
            .into_iter()
            .map(|c| c.map())
            .collect::<Vec<Course>>()),
        Err(e) => return Err((500, format!("DB error to parse: {}", e.to_string()))),
    }
}

#[derive(Deserialize, Serialize)]
struct CourseInfo {
    name: String,
    role: String,
    tests: Vec<Test>,
}

#[derive(Deserialize, Serialize)]
struct Test {
    name: String,
    weight: u16,
}

pub async fn info(course_id: String) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
SELECT
name,
tests,
(SELECT
    role
    FROM <-teaches
    WHERE in = professor:87654321)[0].role
    AS role
FROM ONLY course:kp7n2n27oedkvphduv29;
    "#;

    let mut resp = match DB.query(query).bind(("course_id", course_id)).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    let course = match resp.take::<Option<CourseInfo>>(0) {
        Ok(c) => c,
        Err(e) => return Err((500, format!("DB error to parse: {}", e.to_string()))),
    };

    match course {
        Some(course) => Ok(course),
        None => Err((404, "Course not found".to_string())),
    }
}
