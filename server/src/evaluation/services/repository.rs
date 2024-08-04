use serde::{Deserialize, Serialize};

use super::DB;

pub async fn register_evaluation(
    course_id: &String,
    student_id: &String,
    ev_type: &String,
    score: u8,
    number: u8,
    weight: u8,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
IF (SELECT * FROM register_time WHERE to >= time::now() && for="{}") != []
  {{

		RELATE course:{} -> evaluated -> student:{} SET id = {{
			number: {},
			type: '{}',
      student: '{}',
      course: '{}'
		}}, score = {}, weight = {};

		RETURN 'Evaluation registered';
	
}}
ELSE
  {{
		THROW 'Register no avilable';
	}}
;
    "#,
        number, course_id, student_id, number, ev_type, student_id, course_id, score, weight
    );

    let mut resp = match db.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Option<String>>(0) {
        Ok(r) => match r {
            Some(_) => Ok("Evaluation registered".to_string()),
            None => return Err((500, format!("DB response none"))),
        },

        Err(e) => return Err((400, format!("DB error: {}", e.to_string()))),
    }
}

pub async fn teaches_course(
    profesor_id: &String,
    course_id: &String,
    db: &DB,
) -> Result<bool, (u16, String)> {
    let query = format!(
        r#"
(select role from course:{}<-teaches where in = professor:{})[0].role
"#,
        course_id, profesor_id
    );

    let mut resp = match db.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    let role = match resp.take::<Option<String>>(0) {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error parse: {}", e.to_string()))),
    };

    match role {
        Some(r) => Ok(r == "practice"),
        None => Err((400, format!("You aren't avilable to update note"))),
    }
}

#[derive(Serialize, Deserialize)]
struct Evaluation {
    #[allow(dead_code)]
    ev_type: String,
    score: f32,
    weight: u8,
    number: u8,
}

pub async fn get_evaluation(
    student_id: &String,
    course_id: &String,
    db: &DB,
) -> Result<impl Serialize, (u16, String)> {
    let query = format!(
        r#"
SELECT
id.type AS ev_type,
id.number AS number,
score, weight
FROM student:{}<-evaluated
WHERE in = course:{};
"#,
        student_id, course_id
    );

    let mut resp = match db.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Vec<Evaluation>>(0) {
        Ok(ev) => Ok(ev),
        Err(e) => Err((500, format!("DB error parse: {}", e.to_string()))),
    }
}