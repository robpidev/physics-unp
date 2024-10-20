use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Response};

use crate::shared::repository::db::DB;

pub async fn create(name: &String, faculty_id: &String) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
IF (SELECT * FROM faculty:{}) != [] {{
	(RELATE faculty:{} -> includes -> (CREATE school SET name = '{}')).out.name
}} ELSE {{
	(RETURN NONE)
}}
    "#,
        faculty_id, faculty_id, name
    );

    let mut resp = make_petition(&query).await?;
    let name = match resp.take::<Option<String>>(0) {
        Ok(school) => school,
        Err(_) => return Err((400, format!("DB Error, School Exists"))),
    };

    match name {
        Some(name) => Ok(format!("School created: {}", name)),
        None => Err((400, "Faculty id dont't exist".to_string())),
    }
}

#[derive(Deserialize, Serialize)]
struct SchoolDB {
    id: Thing,
    name: String,
}

#[derive(Deserialize, Serialize)]
struct School {
    id: String,
    name: String,
}

impl SchoolDB {
    fn map(self) -> School {
        School {
            id: self.id.id.to_string(),
            name: self.name,
        }
    }
}

pub async fn get() -> Result<impl Serialize, (u16, String)> {
    let query = format!(r#"SELECT id, name FROM school;"#);

    let mut resp = make_petition(&query).await?;

    parse_school(&mut resp)
}

fn parse_school(resp: &mut Response) -> Result<impl Serialize, (u16, String)> {
    match resp.take::<Vec<SchoolDB>>(0) {
        Ok(schools) => Ok(schools
            .into_iter()
            .map(|s| s.map())
            .collect::<Vec<School>>()),
        Err(e) => return Err((400, format!("DB search error: {}", e.to_string()))),
    }
}

pub async fn delete(id: &String) -> Result<String, (u16, String)> {
    let query = format!(r#"DELETE school:{} RETURN BEFORE;"#, id);

    let mut result = make_petition(&query).await?;

    let school = match result.take::<Option<SchoolDB>>(0) {
        Ok(school) => school,
        Err(e) => return Err((500, format!("DB error: {}", e.to_string()))),
    };
    match school {
        Some(school) => Ok(format!("School deleted: {}", school.name)),
        None => return Err((401, "School don't exist".to_string())),
    }
}

async fn make_petition(query: &String) -> Result<Response, (u16, String)> {
    match DB.query(query).await {
        Ok(resp) => Ok(resp),
        Err(e) => Err((500, format!("DB query error: {}", e.to_string()))),
    }
}

pub async fn get_by_id(id: String) -> Result<impl Serialize, (u16, String)> {
    let query =
        r#"SELECT out.id AS id, out.name AS name FROM type::thing("faculty", $id)->includes;"#;

    let mut resp = match DB.query(query).bind(("id", id)).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB conection error: {}", e.to_string()))),
    };

    parse_school(&mut resp)
}
