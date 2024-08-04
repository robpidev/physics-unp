use super::{last_name::LastName, names::Names};

pub struct Student {
    pub code: String,
    pub names: Names,
    pub last_name1: LastName,
    pub last_name2: LastName,
    pub password: String,
    pub gender: bool,
}

impl Student {
    pub fn new(
        code: String,
        names: String,
        last_name1: String,
        last_name2: String,
        password: String,
        gender: bool,
    ) -> Result<Self, String> {
        if password.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }

        Ok(Self {
            code,
            names: Names::new(names)?,
            last_name1: LastName::new(last_name1)?,
            last_name2: LastName::new(last_name2)?,
            password,
            gender,
        })
    }
}

impl ToString for Student {
    fn to_string(&self) -> String {
        format!(
            r#"{{
  "code": "{}",
  "names": "{}",
  "last_name1": "{}",
  "last_name2": "{}",
  "password": crypto::bcrypt::generate("{}"),
  "gender": {}
}}"#,
            self.code,
            self.names.to_string(),
            self.last_name1.to_string(),
            self.last_name2.to_string(),
            self.password,
            self.gender
        )
    }
}
