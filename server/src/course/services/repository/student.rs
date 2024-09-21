use serde::Serialize;

use super::{Course, CourseDB};
use crate::shared::repository::db::DB;

pub async fn courses(id: &String) -> Result<impl Serialize, (u16, String)> {
    let query = format!(
        r#"
(select <-has<-school->offers.out.* as courses from only student:{id}).courses
"#,
    );

    let mut resp = match DB.query(query).await {
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
