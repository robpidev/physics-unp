use super::Vars;

/// Struct for database config
#[derive(Default, Debug)]
pub struct DBVars {
    pub ws: String,
    pub name_space: String,
    pub name: String,
    pub user: String,
    pub password: String,
}

impl Vars for DBVars {}

impl DBVars {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            ws: DBVars::var("DB_WS")?,
            name_space: DBVars::var("DB_NS")?,
            name: DBVars::var("DB_NAME")?,
            user: DBVars::var("DB_USER")?,
            password: DBVars::var("DB_PASS")?,
        })
    }
}
