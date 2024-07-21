use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct ProfessorDB {
    #[allow(dead_code)]
    names: String,
    last_name1: String,
    last_name2: String,
    dni: String,
    gender: bool,
    role: String,
}

impl ToString for ProfessorDB {
    fn to_string(&self) -> String {
        format!(
            r#"{{"names":"{}","last_name1":"{}","last_name2":"{}","dni":"{}","gender":{},"role":"{}"}}"#,
            self.names, self.last_name1, self.last_name2, self.dni, self.gender, self.role
        )
    }
}
