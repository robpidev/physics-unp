use serde::Serialize;

use super::repository::signin;

pub async fn login(id: String, password: String) -> Result<impl Serialize, (u16, String)> {
    if id.len() == 8 {
        signin::sign_in(id, password, "professor").await
    } else if id.len() == 10 {
        signin::sign_in(id, password, "student").await
    } else {
        Err((400, "Code invalid".to_string()))
    }
}
