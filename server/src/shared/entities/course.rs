use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub name: String,
}
