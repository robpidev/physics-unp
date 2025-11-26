use crate::shared::{entities::user::User, repository::db::DB};
use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Deserialize, Clone)]
struct UserDB {
    id: Thing,
    names: String,
    last_name1: String,
    last_name2: String,
    role: Option<String>,
    gender: bool,
}

//=== Signin Repository ===

pub struct SignInRepository {
    query: &'static str,
    user_type: &'static str,
}

impl SignInRepository {
    fn default(user_type: &'static str) -> Self {
        Self {
            query: "
            (SELECT id, names, last_name1, last_name2, role, gender
            FROM type::thing($table, <int>$id)
            WHERE crypto::bcrypt::compare(password, $password))[0];
            ",
            user_type,
        }
    }

    pub fn new(user_type: &'static str) -> Self {
        Self::default(user_type)
    }

    pub async fn sign_in(&self, id: String, password: String) -> Result<User, (u16, String)> {
        let mut resp = match DB
            .query(self.query)
            .bind(("table", self.user_type))
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
                    format!("DB login error: {}", Self::parse_error(e.to_string())),
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

        Ok(user)
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
}
