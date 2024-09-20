use crate::shared::{
    entities::{professor::ProfessorDB, student::StudentDB},
    repository::db::DB,
};

use super::super::entities::{professor::Professor, student::Student};
use serde::{de::DeserializeOwned, Deserialize};
use surrealdb::{sql::Thing, Error, Response};

async fn response_parse<T: ToString + DeserializeOwned>(
    res: Result<Response, Error>,
) -> Result<String, (u16, String)> {
    let mut resp = match res {
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

pub async fn register_professor(professor: Professor, db: &DB) -> Result<String, (u16, String)> {
    let query = r#"
IF (SELECT * FROM register_time WHERE todo = "professor" AND start < time::now() AND end > time::now()) != []
  {
		RETURN CREATE type::thing('professor', <int>$id) CONTENT {
    names: $names,
    last_name1: $last_name1,
    last_name2: $last_name2,
    dni: $dni,
    password: crypto::bcrypt::generate($password),
    gender: $gender,
    };
	}
ELSE
  {
		THROW 'Register no avilable';
	}
;"#;

    let res = db
        .query(query)
        .bind(("id", professor.dni.clone()))
        .bind(("names", professor.names.to_string()))
        .bind(("last_name1", professor.last_name1.to_string()))
        .bind(("last_name2", professor.last_name2.to_string()))
        .bind(("dni", professor.dni))
        .bind(("password", professor.password))
        .bind(("gender", professor.gender))
        .await;

    response_parse::<ProfessorDB>(res).await
}

pub async fn register_student(
    student: Student,
    school_id: String,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query = r#"
BEGIN TRANSACTION;
IF (SELECT * FROM register_time WHERE todo = 'student' AND start < time::now() AND end > time::now()) != []
  {
        let $s = CREATE type::thing('student', <int>$id) CONTENT {
        names: $names,
        last_name1: $last_name1,
        last_name2: $last_name2,
        code: $code,
        password: crypto::bcrypt::generate($password),
        gender: $gender,
        };
		RELATE (select * from type::thing('school', $school_id)) -> has -> $s;
        RETURN $s;
	}
ELSE
  {
		THROW 'Register no avilable';
	}
;
COMMIT TRANSACTION;
    "#;

    let res = db
        .query(query)
        .bind(("id", student.code.clone()))
        .bind(("names", student.names))
        .bind(("last_name1", student.last_name1.to_string()))
        .bind(("last_name2", student.last_name2.to_string()))
        .bind(("code", student.code))
        .bind(("password", student.password))
        .bind(("gender", student.gender))
        .bind(("school_id", school_id))
        .await;

    response_parse::<StudentDB>(res).await
}

#[derive(Deserialize)]
struct SchoolDB {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn verify_school(school_id: String, db: &DB) -> Result<(), (u16, String)> {
    let query = "select id from type::thing('school', $id)";
    let mut resp = match db.query(query).bind(("id", school_id)).await {
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
