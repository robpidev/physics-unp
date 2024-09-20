use crate::shared::repository::db::DB;
use serde::Serialize;

use super::CourseTest;

pub async fn update(
    course_id: String,
    test: u8,
    weight: u8,
) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
UPDATE type::thing("course", $course_id) SET tests = [
	{
		name: 'test',
		weight: $test
	},
	{
		name: 'practice',
		weight: $weight
	}
];"#;

    let mut resp = match DB
        .query(query)
        .bind(("course_id", course_id))
        .bind(("test", test))
        .bind(("weight", weight))
        .await
    {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB conection Error: {}", e.to_string()))),
    };

    match resp.take::<Option<CourseTest>>(0) {
        Ok(c) => Ok(c),
        Err(e) => Err((500, format!("DB Error parse: {}", e.to_string()))),
    }
}
