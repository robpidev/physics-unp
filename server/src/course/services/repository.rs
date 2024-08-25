use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::shared::entities::{course::Course, user::User};

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
IF (SELECT * FROM school:{}) != [] THEN
	(SELECT out.id AS id, out.name AS name, out.places AS places FROM
  (RELATE school:{} -> offers -> (CREATE course CONTENT {{
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
            id: c.id.id.to_string(),
            name: c.name,
            places: c.places,
        }),
        None => Err((400, format!("School id don't exist: {}", name))),
    }
}

pub async fn delete(id: &String, db: &DB) -> Result<String, (u16, String)> {
    let query = format!("DELETE course:{}  RETURN BEFORE;", id);

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
                id: c.id.id.to_string(),
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
FROM school:{}->offers
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
                id: c.id.id.to_string(),
                name: c.name,
                places: c.places,
            })
            .collect::<Vec<Course>>()),
        Err(e) => Err((500, format!("DB Error: {}", e.to_string()))),
    }
}

#[derive(Deserialize)]
pub struct SchoolDB {
    id: Thing,
    name: String,
}

#[derive(Serialize)]
pub struct School {
    id: String,
    name: String,
}

pub async fn get_by_professor(
    professor_id: &String,
    db: &DB,
) -> Result<impl Serialize, (u16, String)> {
    let query = format!(
        r#"
SELECT out.name AS name, out.id AS id FROM professor:{}->teaches; 
    "#,
        professor_id
    );

    let mut resp = match db.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Vec<SchoolDB>>(0) {
        Ok(schools) => Ok(schools
            .into_iter()
            .map(|s| School {
                id: s.id.id.to_string(),
                name: s.name,
            })
            .collect::<Vec<School>>()),

        Err(e) => Err((500, format!("DB error to parse: {}", e.to_string()))),
    }
}

pub async fn check_id(id: &String, db: &DB) -> Result<bool, (u16, String)> {
    let query = format!("SELECT * FROM ONLY course:{};", id);

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

#[derive(Deserialize)]
pub struct Enroll {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn enroll(
    student_id: &String,
    course_id: &String,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
select id from RELATE student:{}->enrolled->course:{};
"#,
        student_id, course_id
    );

    let mut resp = match db.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Option<Enroll>>(0) {
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
DELETE enrolled where in=student:{} && out=course:{} return before
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
    role: &String,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
if (select in as in from course:{}<-teaches)[0].in != professor:{} {{
     RELATE professor:{}->teaches->course:{} set role = "{}";
     RETURN "created";
}} else {{
    return NONE;
}}
"#,
        course_id, teacher_id, teacher_id, course_id, role
    );

    let mut resp = match db.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Option<String>>(0) {
        Ok(r) => match r {
            Some(_) => Ok(format!(
                "Teacher {} asigned to course {}",
                teacher_id, course_id
            )),
            None => Err((400, format!("Teacher alredy assigned"))),
        },
        Err(e) => Err((500, format!("DB parse error: {}", e.to_string()))),
    }
}

pub async fn desasign_professor(
    course_id: &String,
    teacher_id: &String,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
DELETE teaches where in=professor:{} && out=course:{} return before
"#,
        teacher_id, course_id
    );

    match db.query(query).await {
        Ok(_) => Ok(format!(
            "Teacher {} desasigned from course {}",
            teacher_id, course_id
        )),
        Err(e) => Err((500, format!("DB Error: {}", e.to_string()))),
    }
}

pub async fn get_by_student(id: &String, db: &DB) -> Result<Vec<Course>, (u16, String)> {
    let query = format!(
        r#"
(select <-has<-school->offers.out.* as courses from only student:{id}).courses
"#,
    );

    let mut resp = match db.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    let courses = match resp.take::<Vec<CourseDB>>(0) {
        Ok(c) => c,
        Err(e) => return Err((500, format!("DB parse error: {}", e.to_string()))),
    };

    Ok(courses
        .into_iter()
        .map(|c| Course {
            id: c.id.id.to_string(),
            name: c.name,
            places: c.places,
        })
        .collect::<Vec<Course>>())
}

pub async fn get_enrolled(id: &String, db: &DB) -> Result<impl Serialize, (u16, String)> {
    let query = format!(
        r#"
(select out.* as course from student:{id}->enrolled)[0].course
"#,
    );

    let mut resp = match db.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    let course = match resp.take::<Option<CourseDB>>(0) {
        Ok(c) => c,
        Err(e) => return Err((500, format!("DB parse error: {}", e.to_string()))),
    };

    match course {
        Some(c) => Ok(Course {
            id: c.id.id.to_string(),
            name: c.name,
            places: c.places,
        }),
        None => Err((204, format!("Not enrolled: {}", id))),
    }
}

#[derive(Deserialize)]
pub struct UserDB {
    pub id: Thing,
    pub names: String,
    pub last_name1: String,
    pub last_name2: String,
    pub gender: bool,
    pub role: String,
}

pub async fn get_professors(course_id: &String, db: &DB) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
SELECT
in.id AS id,
in.names AS names,
in.last_name1 AS last_name1,
in.last_name2 AS last_name2,
in.gender AS gender,
role
FROM type::thing("course", $id)<-teaches
"#;

    let mut resp = match db.query(query).bind(("id", course_id)).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    let professors = match resp.take::<Vec<UserDB>>(0) {
        Ok(p) => p,
        Err(e) => return Err((500, format!("DB parse error: {}", e.to_string()))),
    };

    Ok(professors
        .into_iter()
        .map(|p| User {
            id: p.id.id.to_string(),
            names: p.names,
            last_name1: p.last_name1,
            last_name2: p.last_name2,
            gender: p.gender,
            role: p.role,
        })
        .collect::<Vec<User>>())
}
