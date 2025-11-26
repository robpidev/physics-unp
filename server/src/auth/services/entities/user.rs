use crate::shared::entities::user::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserToken {
    pub user: User,
    pub token: String,
}
