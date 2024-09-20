use std::env;

use crate::shared::{entities::user::User, repository::db::DB};
use dotenv::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

//#[derive(Serialize)]
#[derive(Serialize)]
struct Claims {
    user: User,
    exp: usize,
}

#[derive(Deserialize, Clone)]
struct UserDB {
    id: Thing,
    names: String,
    last_name1: String,
    last_name2: String,
    role: Option<String>,
    gender: bool,
}

#[derive(Serialize)]
struct UserToken {
    user: User,
    token: String,
}

//use crate::shared::entities::{professor::ProfessorDB, student::StudentDB};
pub async fn sign_in<'a>(
    id: String,
    password: String,
    user_type: &'static str,
    db: &DB,
) -> Result<impl Serialize, (u16, String)> {
    let query = "
(SELECT
id, names, last_name1, last_name2, role, gender
FROM type::thing($table, <int>$id)
WHERE crypto::bcrypt::compare(password, $password))[0];
";
    let mut resp = match db
        .query(query)
        .bind(("table", user_type))
        .bind(("id", id))
        .bind(("password", password))
        .await
    {
        Ok(resp) => resp,
        Err(e) => return Err((500, format!("DB conection error: {}", e.to_string()))),
    };

    let user_result = match resp.take::<Option<UserDB>>(0) {
        Ok(professor) => professor,
        Err(e) => {
            return Err((
                400,
                format!("DB login error: {}", parse_error(e.to_string())),
            ))
        }
    };

    let user = match user_result {
        Some(u) => User {
            id: u.id.id.to_string(),
            names: u.names,
            last_name1: u.last_name1,
            last_name2: u.last_name2,
            role: u.role.unwrap_or("student".to_string()),
            gender: u.gender,
        },
        None => return Err((401, "User or password invalid".to_string())),
    };

    let claims = Claims {
        user: user.clone(),
        exp: 30,
    };

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

    Ok(UserToken { user, token })
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
