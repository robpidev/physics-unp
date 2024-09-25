use crate::shared::repository::db::DB;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Evaluation {
    ev_type: String,
    number: u16,
    score: u8,
}

#[derive(Serialize, Deserialize)]
struct Tests {
    name: String,
    weight: u16,
}

#[derive(Serialize, Deserialize)]
struct Course {
    name: String,
    tests: Vec<Tests>,
    evaluations: Vec<Evaluation>,
}

pub async fn evaluation(
    student_id: &String,
    course_id: &String,
) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
SELECT ->evaluated[WHERE out = type::thing('student', <int>$student_id)].{
	ev_type,
	number,
	score
} AS evaluations,
name,
tests
FROM ONLY type::thing("course", $course_id);
    "#;

    let mut resp = match DB
        .query(query)
        .bind(("student_id", student_id.to_string()))
        .bind(("course_id", course_id.to_string()))
        .await
    {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB conection error: {}", e.to_string()))),
    };
    let courses = match resp.take::<Option<Course>>(0) {
        Ok(c) => c,
        Err(e) => return Err((500, format!("DB response error: {}", e.to_string()))),
    };

    match courses {
        Some(c) => Ok(c),
        None => Err((400, format!("Course don't exists"))),
    }
}
