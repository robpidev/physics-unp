use serde::de::DeserializeOwned;

use crate::shared::entitities::{professor::ProfessorDB, student::StudentDB};
use crate::shared::repository::db::DB;
pub async fn sign_in(
    id: String,
    password: String,
    user_type: &str,
    db: &DB,
) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
(SELECT
*
FROM {}:{}
WHERE crypto::bcrypt::compare(password, '{}'))[0];
"#,
        user_type, id, password
    );

    if user_type == "professor" {
        login::<ProfessorDB>(query, db).await
    } else {
        login::<StudentDB>(query, db).await
    }
}

async fn login<T: ToString + DeserializeOwned>(
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
                format!("DB login error: {}", parse_error(e.to_string())),
            ))
        }
    };

    match professor {
        Some(professor) => Ok(professor.to_string()),
        None => Err((500, "User or passwrod invalid".to_string())),
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
