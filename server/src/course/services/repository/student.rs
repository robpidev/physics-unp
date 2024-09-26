use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::shared::repository::db::DB;

#[derive(Deserialize)]
struct CourseDB {
    id: Thing,
    name: String,
}

#[derive(Serialize)]
struct Course {
    id: String,
    name: String,
}

impl CourseDB {
    fn map(self) -> Course {
        Course {
            id: self.id.id.to_string(),
            name: self.name,
        }
    }
}

pub async fn enrolled(student_id: String) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
SELECT id, name
FROM type::thing('student', <int>$student_id)->enrolled->course;
    "#;

    let mut resp = match DB.query(query).bind(("student_id", student_id)).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    let courses = match resp.take::<Vec<CourseDB>>(0) {
        Ok(c) => c,
        Err(e) => return Err((500, format!("DB parse error: {}", e.to_string()))),
    };

    Ok(courses
        .into_iter()
        .map(|c| c.map())
        .collect::<Vec<Course>>())
}

/// This función return all avilable courses for student registration
/// returns a vec con tha nex structure:
///
/// ```rust
/// struct Course {
///     id: String,
///     name: String,
///     places: u16,
///     enrolled: u16,
/// }
/// ```

#[derive(Deserialize)]
struct CourseAvilableDB {
    id: Thing,
    name: String,
    places: u16,
    enrolled: u16,
}

#[derive(Serialize)]
struct CourseAvilable {
    id: String,
    name: String,
    places: u16,
    enrolled: u16,
}

impl CourseAvilableDB {
    fn map(self) -> CourseAvilable {
        CourseAvilable {
            id: self.id.id.to_string(),
            name: self.name,
            places: self.places,
            enrolled: self.enrolled,
        }
    }
}

pub async fn avilables(student_id: String) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
SELECT id, name, places, count(<-enrolled) AS enrolled
FROM type::thing("student", <int>$student_id)<-has<-school->offers->course;
    "#;

    let mut resp = match DB.query(query).bind(("student_id", student_id)).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    let courses = match resp.take::<Vec<CourseAvilableDB>>(0) {
        Ok(c) => c,
        Err(e) => return Err((500, format!("DB parse error: {}", e.to_string()))),
    };

    Ok(courses
        .into_iter()
        .map(|c| c.map())
        .collect::<Vec<CourseAvilable>>())
}

pub async fn enroll(student_id: String, course_id: String) -> Result<String, (u16, String)> {
    let query = r#"
IF (
    SELECT count(<-enrolled) < places AS places
    FROM ONLY type::thing("course", $course_id)).places 
{
	RELATE 
    (type::thing('student', <int> $student_id)) 
    -> enrolled -> 
    (type::thing('course', $course_id))
    ;
}
ELSE
{
	THROW 'All places are ocupades or course don\'t exist';
};
"#;

    let resp = match DB
        .query(query)
        .bind(("course_id", course_id))
        .bind(("student_id", student_id))
        .await
    {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Connect Error: {}", e.to_string()))),
    };

    match resp.check() {
        Ok(_) => return Ok(format!("Course enrolled")),
        Err(e) => return Err((400, format!("DB relate error: {}", e.to_string()))),
    }
}
