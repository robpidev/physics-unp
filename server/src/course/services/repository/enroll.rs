use crate::shared::repository::db::DB;
use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Deserialize)]
pub struct Enroll {
    #[allow(dead_code)]
    id: Thing,
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

// TODO: Add enroll only in his school
pub async fn enroll(student_id: String, course_id: String) -> Result<String, (u16, String)> {
    let query = r#"
IF (
    SELECT count(<-enrolled) < places AS places
    FROM ONLY type::thing('course', $course_id)
).places {
	IF count(
        SELECT id FROM enrolled
                WHERE out = type::thing('course', $course_id)
                AND in = type::thing('student', <int> $student_id)
            ) == 0 {
                RELATE (type::thing('student', <int> $student_id))
                -> enrolled ->
                (type::thing('course', $course_id));
			} else {
                THROW 'Already enrolled';
            };
	} ELSE {
		THROW 'All places ocupateds o course dont\' exists';
}
;
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
