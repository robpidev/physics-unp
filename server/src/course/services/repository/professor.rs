use crate::shared::repository::db::DB;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Deserialize)]
struct CourseDB {
    role: String,
    name: String,
    school: String,
    id: Thing,
}

#[derive(Serialize)]
struct Course {
    role: String,
    name: String,
    school: String,
    id: String,
}

impl CourseDB {
    fn map(self) -> Course {
        Course {
            role: self.role,
            name: self.name,
            school: self.school,
            id: self.id.id.to_string(),
        }
    }
}

pub async fn courses(professor_id: String) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
SELECT role, out.name AS name, out.id AS id,
(->course<-offers<-school.name)[0] as school 
FROM type::thing("professor", <int>$professor_id)->teaches;
    "#;

    let mut resp = match DB.query(query).bind(("professor_id", professor_id)).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Vec<CourseDB>>(0) {
        Ok(courses) => Ok(courses
            .into_iter()
            .map(|c| c.map())
            .collect::<Vec<Course>>()),
        Err(e) => return Err((500, format!("DB error to parse: {}", e.to_string()))),
    }
}
