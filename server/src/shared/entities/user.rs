use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    #[allow(dead_code)]
    pub id: String,
    pub names: String,
    pub last_name1: String,
    pub last_name2: String,
    pub gender: bool,
    pub role: String,
}
