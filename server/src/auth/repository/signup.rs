use crate::shared::{
    entities::{professor::ProfessorDB, school::School, student::StudentDB},
    repository::db::DB,
};
use serde::{de::DeserializeOwned, Deserialize};
use surrealdb::sql::Thing;

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

fn query_professor(person: String, id: String, person_type: String) -> String {
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

fn query_student(person: String, id: String, school_id: &String) -> String {
    format!(
        r#"
BEGIN TRANSACTION;
IF (SELECT * FROM register_time WHERE for = 'student' AND from < time::now() AND to > time::now()) != []
  {{
        let $s = CREATE student:{} CONTENT {};
		RELATE (select * from {}) -> has -> $s;
        RETURN $s
	}}
ELSE
  {{
		THROW 'Register no avilable';
	}}
;
COMMIT TRANSACTION;
    "#,
        id, person, school_id
    )
}

pub async fn save<T: ToString>(
    person: T,
    id: String,
    user_type: String,
    school_id: &String,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query;
    if user_type == "professor" {
        query = query_professor(person.to_string(), id, user_type.clone());
        register::<ProfessorDB>(query, db).await
    } else if user_type == "student" {
        query = query_student(person.to_string(), id, school_id);
        register::<StudentDB>(query, db).await
    } else {
        Err((400, "User type not valid".to_string()))
    }
}

#[derive(Deserialize)]
struct SchoolDB {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn verify_school(school_id: &String, db: &DB) -> Result<(), (u16, String)> {
    let query = format!("select id from {}", school_id);
    let mut resp = match db.query(query).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB conection error: {}", e.to_string()))),
    };

    let fac = match resp.take::<Option<SchoolDB>>(0) {
        Ok(f) => f,
        Err(e) => {
            return Err((
                400,
                format!("DB parse error: {}", parse_error(e.to_string())),
            ))
        }
    };

    match fac {
        Some(_) => Ok(()),
        None => Err((400, "School id not valid".to_string())),
    }
}
