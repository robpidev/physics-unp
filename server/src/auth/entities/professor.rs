use super::last_name::LastName;
use super::names::Names;

pub struct Professor {
    names: Names,
    last_name1: LastName,
    last_name2: LastName,
    dni: String,
    password: String,
    gender: bool,
}

impl Professor {
    pub fn new(
        names: String,
        last_name1: String,
        last_name2: String,
        dni: String,
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

    pub fn to_json(&self) -> String {
        format!(
            r#"{{
  "names": "{}",
  "last_name1": "{}",
  "last_name2": "{}",
  "dni": "{}",
  "password": "{}",
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

    pub fn get_dni(&self) -> String {
        self.dni.clone()
    }
}

impl ToString for Professor {
    fn to_string(&self) -> String {
        format!(
            r#"{{"names":"{}","last_name1":"{}","last_name2":"{}","dni":"{}", password:"{}"}}"#,
            self.names.to_string(),
            self.last_name1.to_string(),
            self.last_name2.to_string(),
            self.dni,
            self.password
        )
    }
}
