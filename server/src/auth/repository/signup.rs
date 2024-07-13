use crate::auth::entities::professor::Professor;
use crate::auth::entities::student::Student;
use crate::shared::repository::db::DB;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct CreatedProfessor {
    #[allow(dead_code)]
    names: String,
    last_name1: String,
    last_name2: String,
    dni: String,
    gender: bool,
}

impl ToString for CreatedProfessor {
    fn to_string(&self) -> String {
        format!(
            r#"{{
  "names":"{}",
  "last_name1":"{}",
  "last_name2":"{}",
  "dni":"{}",
  "gender":{}
}}"#,
            self.names, self.last_name1, self.last_name2, self.dni, self.gender
        )
    }
}

#[derive(Deserialize, Serialize)]
struct CreatedStudent {
    #[allow(dead_code)]
    code: String,
    names: String,
    last_name1: String,
    last_name2: String,
    gender: bool,
}

impl ToString for CreatedStudent {
    fn to_string(&self) -> String {
        format!(
            r#"{{
  "code":"{}",
  "names":"{}",
  "last_name1":"{}",
  "last_name2":"{}",
  "gender":{}
}}"#,
            self.code, self.names, self.last_name1, self.last_name2, self.gender
        )
    }
}

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

pub async fn register_studend(student: Student, db: &DB) -> Result<String, (u16, String)> {
    let query = format!(
        "CREATE student:{} CONTENT {}",
        student.get_code(),
        student.to_json()
    );

    register::<CreatedStudent>(query, db).await
}

pub async fn register_professor(professor: Professor, db: &DB) -> Result<String, (u16, String)> {
    let query = format!(
        "CREATE professor:{} CONTENT {}",
        professor.get_dni(),
        professor.to_json()
    );

    register::<CreatedProfessor>(query, db).await
}
