use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct School {
    pub id: String,
    pub name: String,
}
