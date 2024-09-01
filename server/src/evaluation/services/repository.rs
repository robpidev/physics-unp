use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::DB;

pub async fn register_evaluation(
    course_id: &String,
    student_id: &String,
    ev_type: &String,
    score: f32,
    number: u8,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query = r#"
IF (SELECT * FROM register_time WHERE to >= time::now() && for=type::string($number)) != [] {
    let $course = type::thing("course", $course_id);
    let $student = type::thing("student", <int>$student_id);
    RELATE $course->evaluated->$student
    SET
    ev_type = $ev_type,
    number = $number,
    score = $score;
    RETURN 'Evaluation registered';
	
} ELSE {
		THROW 'Register no avilable';
	}
;"#;

    let mut resp = match db
        .query(query)
        .bind(("course_id", course_id))
        .bind(("student_id", student_id))
        .bind(("ev_type", ev_type))
        .bind(("score", score))
        .bind(("number", number))
        .await
    {
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

pub async fn update_evaluation(
    ev_id: &String,
    score: f32,
    number: u8,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query = r#"
IF (SELECT * FROM register_time WHERE to >= time::now() && for=type::string($number)) != [] {
    UPDATE type::thing("evaluated", $ev_id) set score = $score;
    RETURN 'Evaluation updated';
} ELSE {
        THROW 'Update no avilable';
}
    "#;

    let mut resp = match db
        .query(query)
        .bind(("ev_id", ev_id))
        .bind(("score", score))
        .bind(("number", number))
        .await
    {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB error: {}", e.to_string()))),
    };

    match resp.take::<Option<String>>(0) {
        Ok(r) => match r {
            Some(m) => Ok(m),
            None => Err((500, format!("DB response none"))),
        },

        Err(e) => Err((400, format!("DB response error: {}", e.to_string()))),
    }
}

pub async fn teaches_course(
    professor_id: &String,
    course_id: &String,
    db: &DB,
) -> Result<bool, (u16, String)> {
    let query = format!(
        r#"
(select role from type::thing("course", $course_id)<-teaches
where in = type::thing("professor", type::int($professor_id)))[0].role
"#,
    );

    let mut resp = match db
        .query(query)
        .bind(("course_id", course_id))
        .bind(("professor_id", professor_id))
        .await
    {
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
score
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

#[derive(Serialize, Deserialize)]
struct ScoreDB {
    id: Thing,
    ev_type: String,
    number: u8,
    score: f32,
}

// TODO: Filter in db
#[derive(Serialize, Deserialize)]
struct EvaluationDB {
    id: Thing,
    name: String,
    scores: Vec<ScoreDB>,
}

#[derive(Serialize)]
struct Score {
    id: String,
    number: u8,
    score: f32,
    ev_type: String,
}

#[derive(Serialize)]
struct EvaluationsCourse {
    name: String,
    id: String,
    scores: Vec<Score>,
}

pub async fn get_all_evaluations_course(
    course_id: &String,
    db: &DB,
) -> Result<impl Serialize, (u16, String)> {
    let query = r#"select <-student<-evaluated.*  as scores,
in.names + ' ' + in.last_name1 + ' ' + in.last_name2 as name,
in.id as id
omit scores.in, scores.out
from type::thing("course", $course_id)<-enrolled
"#;

    let mut resp = match db.query(query).bind(("course_id", course_id)).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    let evaluations = match resp.take::<Vec<EvaluationDB>>(0) {
        Ok(ev) => ev,
        Err(e) => return Err((500, format!("DB error parse: {}", e.to_string()))),
    };

    Ok(evaluations
        .into_iter()
        .map(|e| EvaluationsCourse {
            id: e.id.id.to_string(),
            name: e.name,
            scores: e
                .scores
                .into_iter()
                .map(|s| Score {
                    id: s.id.id.to_string(),
                    number: s.number,
                    score: s.score,
                    ev_type: s.ev_type,
                })
                .collect::<Vec<Score>>(),
        })
        .collect::<Vec<EvaluationsCourse>>())
}
