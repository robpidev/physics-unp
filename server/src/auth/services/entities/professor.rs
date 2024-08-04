use super::last_name::LastName;
use super::names::Names;

pub struct Professor {
    pub names: Names,
    pub last_name1: LastName,
    pub last_name2: LastName,
    pub dni: String,
    pub password: String,
    pub gender: bool,
}

impl Professor {
    pub fn new(
        dni: String,
        names: String,
        last_name1: String,
        last_name2: String,
        password: String,
        gender: bool,
    ) -> Result<Self, String> {
        let names = Names::new(names)?;
        let last_name1 = LastName::new(last_name1)?;
        let last_name2 = LastName::new(last_name2)?;

        if password.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }

        Ok(Self {
            names,
            last_name1,
            last_name2,
            dni,
            password,
            gender,
        })
    }
}

impl ToString for Professor {
    fn to_string(&self) -> String {
        format!(
            r#"{{
  "names": "{}",
  "last_name1": "{}",
  "last_name2": "{}",
  "dni": "{}",
  "password": crypto::bcrypt::generate("{}"),
  "gender": {}
}}"#,
            self.names.to_string(),
            self.last_name1.to_string(),
            self.last_name2.to_string(),
            self.dni,
            self.password,
            self.gender,
        )
    }
}
