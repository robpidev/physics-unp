use crate::shared::repository::db::DB;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Deserialize)]
pub struct SchoolDB {
    id: Thing,
    name: String,
    school: String,
}

#[derive(Serialize)]
pub struct School {
    id: String,
    name: String,
    school: String,
}

impl SchoolDB {
    fn map(self) -> School {
        School {
            id: self.id.id.to_string(),
            name: self.name,
            school: self.school,
        }
    }
}

pub async fn courses(professor_id: String) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
SELECT (->course<-offers<-school.name)[0] as school,
out.name AS name,
out.id AS id 
FROM type::thing("professor", <int>$professor_id)->teaches;
    "#;

    let mut resp = match DB.query(query).bind(("professor_id", professor_id)).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Vec<SchoolDB>>(0) {
        Ok(schools) => Ok(schools
            .into_iter()
            .map(|s| s.map())
            .collect::<Vec<School>>()),

        Err(e) => Err((500, format!("DB error to parse: {}", e.to_string()))),
    }
}
