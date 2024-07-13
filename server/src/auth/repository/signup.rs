use crate::auth::entities::professor::Professor;
use crate::shared::repository::db::DB;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreatedProfessor {
    #[allow(dead_code)]
    names: String,
    last_name1: String,
    last_name2: String,
    dni: String,
}

impl ToString for CreatedProfessor {
    fn to_string(&self) -> String {
        format!(
            r#"{{
  "names":"{}",
  "last_name1":"{}",
  "last_name2":"{}",
  "dni":"{}"
}}"#,
            self.names, self.last_name1, self.last_name2, self.dni
        )
    }
}

pub async fn create_professor(professor: Professor, db: &DB) -> Result<String, (u16, String)> {
    let query = format!(
        "CREATE professor:{} CONTENT {}",
        professor.get_dni(),
        professor.to_json()
    );

    let mut resp = match db.query(query).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB conection error: {}", e.to_string()))),
    };

    let professor = match resp.take::<Option<CreatedProfessor>>(0) {
        Ok(professor) => professor,
        Err(e) => {
            return Err((
                500,
                format!("Register error: {}", parse_error(e.to_string())),
            ))
        }
    };

    if let Some(professor) = professor {
        return Ok(professor.to_string());
    } else {
        return Err((500, "Register error".to_string()));
    }
}

fn parse_error(error: String) -> String {
    if error.contains("already exists") {
        return "DNI already exists".to_string();
    } else {
        return error;
    }
}
