use crate::shared::repository::db::DB;
use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Deserialize)]
pub struct Enroll {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn new(student_id: &String, course_id: &String) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
select id from RELATE student:{}->enrolled->course:{};
"#,
        student_id, course_id
    );

    let mut resp = match DB.query(query).await {
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

pub async fn unregister(student_id: &String, course_id: &String) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
DELETE enrolled where in=student:{} && out=course:{} return before
"#,
        student_id, course_id
    );

    match DB.query(query).await {
        Ok(_) => Ok(format!(
            "Student {} unregistered from course {}",
            student_id, course_id
        )),
        Err(e) => Err((500, format!("DB Error: {}", e.to_string()))),
    }
}
