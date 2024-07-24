use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Faculty {
    pub id: String,
    pub name: String,
}
