use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::shared::entities::user::User;

#[derive(Deserialize)]
pub struct ProfessorDB {
    id: Thing,
    names: String,
    last_name1: String,
    last_name2: String,
    gender: bool,
    role: String,
}

pub async fn get_all(db: &super::DB) -> Result<impl Serialize, (u16, String)> {
    let query = format!("SELECT * FROM professor;");
    let mut resp = match db.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Vec<ProfessorDB>>(0) {
        Ok(professors) => Ok(professors
            .into_iter()
            .map(|p| User {
                id: p.id.id.to_string(),
                names: p.names,
                last_name1: p.last_name1,
                last_name2: p.last_name2,
                gender: p.gender,
                role: p.role,
            })
            .collect::<Vec<User>>()),
        Err(e) => return Err((500, format!("DB response error: {}", e.to_string()))),
    }
}
