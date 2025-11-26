use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

use crate::{config::SeedJwtVar, shared::entities::Claims};

use super::entities::UserToken;
use super::repository::SignInRepository;

pub async fn login(id: String, password: String) -> Result<impl Serialize, (u16, String)> {
    let rep;

    if id.len() == 8 {
        rep = SignInRepository::new("professor");
    } else if id.len() == 10 {
        rep = SignInRepository::new("student");
    } else {
        return Err((400, "User not found".to_string()));
    };

    let user = rep.sign_in(id, password).await?;

    let claims = Claims {
        data: user.clone(),
        exp: 0,
    };

    let secret = match SeedJwtVar::from_env() {
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
