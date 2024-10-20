use serde::{Deserialize, Serialize};

use crate::shared::{entities::faculty::Faculty, repository::db::DB};

use surrealdb::sql::Thing;

#[derive(Deserialize, Serialize)]
struct FacultyDB {
    #[allow(dead_code)]
    id: Thing,
    name: String,
}

impl ToString for FacultyDB {
    fn to_string(&self) -> String {
        format!("Faculty created: {}", self.name)
    }
}

pub async fn create(name: &String) -> Result<String, (u16, String)> {
    let query = format!(r#"CREATE faculty SET name = "{}";"#, name);

    let mut resp = match DB.query(&query).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB conection error: {}", e.to_string()))),
    };

    let faculty = match resp.take::<Option<FacultyDB>>(0) {
        Ok(faculty) => faculty,
        Err(e) => {
            return Err((
                400,
                format!("DB register error: {}", parse_error(e.to_string())),
            ))
        }
    };

    match faculty {
        Some(faculty) => Ok(faculty.to_string()),
        None => Err((500, "DB Response error".to_string())),
    }
}

fn parse_error(error: String) -> String {
    if error.contains("index_name") {
        return "Faculty exists".to_string();
    } else {
        return error;
    }
}

pub async fn get() -> Result<impl Serialize, (u16, String)> {
    let query = format!(r#"SELECT id, name FROM faculty;"#);

    let mut resp = match DB.query(&query).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB conection error: {}", e.to_string()))),
    };

    let faculties = match resp.take::<Vec<FacultyDB>>(0) {
        Ok(faculties) => faculties
            .into_iter()
            .map(|f| Faculty {
                id: f.id.id.to_string(),
                name: f.name,
            })
            .collect::<Vec<Faculty>>(),
        Err(e) => {
            return Err((
                400,
                format!("DB search error: {}", parse_error(e.to_string())),
            ))
        }
    };

    //match faculties {
    //    Some(faculty) => Ok(faculty),
    //    None => Err((500, "No faculties".to_string())),
    //}

    Ok(faculties)
}

pub async fn delete(id: &String) -> Result<String, (u16, String)> {
    let faculty: Option<FacultyDB> = match DB.delete(("faculty", id)).await {
        Ok(faculty) => faculty,
        Err(e) => return Err((500, format!("DB conection error: {}", e.to_string()))),
    };

    match faculty {
        Some(faculty) => Ok(format!("Faculty {} delete", faculty.name)),
        None => Err((500, "DB Response error".to_string())),
    }
}
