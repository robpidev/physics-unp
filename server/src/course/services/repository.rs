use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::shared::entities::course::Course;

use super::DB;

#[derive(Serialize, Deserialize)]
struct CourseDB {
    id: Thing,
    places: u16,
    name: String,
}

pub async fn create(
    name: &String,
    places: u16,
    school_id: &String,
    db: &DB,
) -> Result<impl Serialize, (u16, String)> {
    let query = format!(
        r#"
BEGIN TRANSACTION;
IF (SELECT * FROM {}) != [] THEN
	(SELECT out.id AS id, out.name AS name, out.places AS places FROM
  (RELATE {} -> offers -> (CREATE course CONTENT {{
		name: "{}",
		places: {},
	}})))
ELSE
	(RETURN NONE)
END;
COMMIT TRANSACTION;
    "#,
        school_id, school_id, name, places
    );

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
            places: c.places,
        }),
        None => Err((400, format!("School id don't exist: {}", name))),
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
                places: c.places,
            })
            .collect::<Vec<Course>>()),
        Err(e) => Err((500, format!("DB Error: {}", e.to_string()))),
    }
}

pub async fn get_by_school(school_id: &String, db: &DB) -> Result<impl Serialize, (u16, String)> {
    let query = format!(
        r#"
SELECT
out.id as id, out.name as name, out.places as places
FROM {}->offers
        "#,
        school_id,
    );

    let mut resp = match db.query(query).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Vec<CourseDB>>(0) {
        Ok(resp) => Ok(resp
            .into_iter()
            .map(|c| Course {
                id: c.id.to_string(),
                name: c.name,
                places: c.places,
            })
            .collect::<Vec<Course>>()),
        Err(e) => Err((500, format!("DB Error: {}", e.to_string()))),
    }
}

pub async fn check_id(id: &String, db: &DB) -> Result<bool, (u16, String)> {
    let query = format!("SELECT * FROM ONLY {};", id);

    let mut resp = match db.query(query).await {
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

pub async fn enroll(
    student_id: &String,
    course_id: &String,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
RELATE {}->enrolled->{};
"#,
        student_id, course_id
    );

    let mut resp = match db.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Option<CourseDB>>(0) {
        Ok(c) => match c {
            Some(_) => Ok(format!(
                "Student {} enrolled in course {}",
                student_id, course_id
            )),
            None => Err((400, format!("DB resp None"))),
        },
        Err(e) => Err((400, format!("Student already enrolled: {}", e.to_string()))),
    }
}

pub async fn unregister(
    student_id: &String,
    course_id: &String,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
DELETE enrolled where in={} && out={} return before
"#,
        student_id, course_id
    );

    match db.query(query).await {
        Ok(_) => Ok(format!(
            "Student {} unregistered from course {}",
            student_id, course_id
        )),
        Err(e) => Err((500, format!("DB Error: {}", e.to_string()))),
    }
}

pub async fn asign_professor(
    course_id: &String,
    teacher_id: &String,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
RELATE {}->teaches->{};
"#,
        course_id, teacher_id
    );

    let mut resp = match db.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Option<CourseDB>>(0) {
        Ok(c) => match c {
            Some(_) => Ok(format!(
                "Teacher {} asigned to course {}",
                teacher_id, course_id
            )),
            None => Err((400, format!("DB resp None"))),
        },
        Err(e) => Err((400, format!("Teacher already asigned: {}", e.to_string()))),
    }
}

pub async fn desasign_professor(
    course_id: &String,
    teacher_id: &String,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
DELETE teaches where in={} && out={} return before
"#,
        course_id, teacher_id
    );

    match db.query(query).await {
        Ok(_) => Ok(format!(
            "Teacher {} desasigned from course {}",
            teacher_id, course_id
        )),
        Err(e) => Err((500, format!("DB Error: {}", e.to_string()))),
    }
}
