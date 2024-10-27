use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::shared::repository::db::DB;

#[derive(Deserialize)]
struct ProfessorsDB {
    full_name: String,
    id: Thing,
    role: String,
}

#[derive(Deserialize, Serialize)]
struct Test {
    name: String,
    weight: u8,
}

#[derive(Deserialize)]
struct CourseDB {
    enrolled: u16,
    id: Thing,
    name: String,
    places: u16,
    professors: Vec<ProfessorsDB>,
    tests: Vec<Test>,
}

#[derive(Serialize)]
struct Professors {
    id: String,
    role: String,
    full_name: String,
}

#[derive(Serialize)]
struct Course {
    id: String,
    name: String,
    places: u16,
    enrolled: u16,
    professors: Vec<Professors>,
    tests: Vec<Test>,
}

impl CourseDB {
    fn map(self) -> Course {
        Course {
            id: self.id.id.to_string(),
            name: self.name,
            places: self.places,
            enrolled: self.enrolled,
            professors: self
                .professors
                .into_iter()
                .map(|p| Professors {
                    id: p.id.id.to_string(),
                    full_name: p.full_name,
                    role: p.role,
                })
                .collect(),
            tests: self.tests,
        }
    }
}

pub async fn course(course_id: &String) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
SELECT *,
(SELECT role, in.id AS id,
in.names + ' ' + in.last_name1 + ' ' + in.last_name2 AS full_name
FROM <-teaches) AS professors,
count(<-enrolled) as enrolled
FROM only type::thing("course", $course_id);
    "#;

    let mut resp = match DB
        .query(query)
        .bind(("course_id", course_id.to_string()))
        .await
    {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    let course = match resp.take::<Option<CourseDB>>(0) {
        Ok(c) => c,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match course {
        Some(c) => Ok(c.map()),
        None => Err((400, format!("Course dont exists: {}", course_id))),
    }
}

pub async fn desasign(course_id: &String, teacher_id: &String) -> Result<String, (u16, String)> {
    let query = r#"
DELETE teaches
WHERE
in = type::thing('professor', <int> $teacher_id)
AND
out = type::thing('course', $course_id) RETURN BEFORE;
"#;

    match DB
        .query(query)
        .bind(("course_id", course_id.to_string()))
        .bind(("teacher_id", teacher_id.to_string()))
        .await
    {
        Ok(_) => Ok(format!(
            "Teacher {} desasigned from course {}",
            teacher_id, course_id
        )),
        Err(e) => Err((500, format!("DB Error: {}", e.to_string()))),
    }
}

pub async fn asign(
    course_id: String,
    professor_id: String,
    role: String,
) -> Result<String, (u16, String)> {
    let query = r#"
IF (SELECT in AS in FROM type::thing("course", $course_id)<-teaches)[0].in != type::thing("professor", $professor_id)
	{{
		RELATE (type::thing("professor", <int>$professor_id))->teaches->(type::thing("course", $course_id))
    SET role=$role;
	}}
ELSE
	{{
		THROW 'Professor aleredy assigned to course';
	}}

"#;

    let mut resp = match DB
        .query(query)
        .bind(("course_id", course_id))
        .bind(("professor_id", professor_id))
        .bind(("role", role))
        .await
    {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Option<String>>(0) {
        Ok(_) => Ok("Teacher assigned".to_string()),
        Err(e) => Err((500, format!("DB parse error: {}", e.to_string()))),
    }
}
