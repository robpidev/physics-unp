use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct StudentDB {
    #[allow(dead_code)]
    code: String,
    names: String,
    last_name1: String,
    last_name2: String,
    gender: bool,
}

impl ToString for StudentDB {
    fn to_string(&self) -> String {
        format!(
            r#"{{"id":"{}","names":"{}","last_name1":"{}","last_name2":"{}","gender":{},"role":"student"}}"#,
            self.code, self.names, self.last_name1, self.last_name2, self.gender
        )
    }
}

impl StudentDB {
    pub fn get_id(&self) -> String {
        self.code.clone()
    }
}
