use crate::shared::{
    entitities::{professor::ProfessorDB, student::StudentDB},
    repository::db::DB,
};
use serde::de::DeserializeOwned;

async fn register<T: ToString + DeserializeOwned>(
    query: String,
    db: &DB,
) -> Result<String, (u16, String)> {
    let mut resp = match db.query(query).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB conection error: {}", e.to_string()))),
    };

    let professor = match resp.take::<Option<T>>(0) {
        Ok(professor) => professor,
        Err(e) => {
            return Err((
                400,
                format!("DB register error: {}", parse_error(e.to_string())),
            ))
        }
    };

    match professor {
        Some(professor) => Ok(professor.to_string()),
        None => Err((500, "DB Response error".to_string())),
    }
}

fn parse_error(error: String) -> String {
    if error.contains("already exists") {
        return "User exists".to_string();
    } else if error.contains("string::is::numeric") {
        return "Code or dni no valid".to_string();
    } else {
        return error;
    }
}

fn create_query(person: String, id: String, person_type: String) -> String {
    format!(
        r#"
IF (SELECT * FROM register_time WHERE for = "{}" AND from < time::now() AND to > time::now()) != []
  {{
		RETURN CREATE {}:{} CONTENT {};
	}}
ELSE
  {{
		THROW 'Register no avilable';
	}}
;"#,
        person_type, person_type, id, person
    )
}

pub async fn save<T: ToString>(
    person: T,
    id: String,
    user_type: String,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query = create_query(person.to_string(), id, user_type.clone());

    if user_type == "professor" {
        register::<ProfessorDB>(query, db).await
    } else if user_type == "student" {
        register::<StudentDB>(query, db).await
    } else {
        Err((400, "User type not valid".to_string()))
    }
}
