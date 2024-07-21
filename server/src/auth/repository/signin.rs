use std::env;
use std::fmt::format;

use dotenv::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

use serde::de::DeserializeOwned;

#[derive(Serialize)]
struct Claims<T> {
    sub: T,
    exp: usize,
}

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
    } else if user_type == "student" {
        login::<StudentDB>(query, db).await
    } else {
        Err((400, "Invalid user type".to_string()))
    }
}

async fn login<T: ToString + Serialize + DeserializeOwned>(
    query: String,
    db: &DB,
) -> Result<String, (u16, String)> {
    let mut resp = match db.query(query).await {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB conection error: {}", e.to_string()))),
    };

    let user_result = match resp.take::<Option<T>>(0) {
        Ok(professor) => professor,
        Err(e) => {
            return Err((
                400,
                format!("DB login error: {}", parse_error(e.to_string())),
            ))
        }
    };

    let user = match user_result {
        Some(u) => u,
        None => return Err((401, "User or password invalid".to_string())),
    };

    let user_str = user.to_string();
    let claims = Claims { sub: user, exp: 30 };

    dotenv().ok();
    let secret = match env::var("SEED_JWT") {
        Ok(s) => s,
        Err(e) => return Err((500, format!("Error getting secret: {}", e.to_string()))),
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(e) => return Err((500, format!("Error encoding token: {}", e.to_string()))),
    };

    Ok(format!(r#"{{"user":{},"token":"{}"}}"#, user_str, token))
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
