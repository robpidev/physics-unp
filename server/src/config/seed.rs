use super::Vars;

pub struct SeedJwtVar;

impl Vars for SeedJwtVar {}

impl SeedJwtVar {
    pub fn from_env() -> Result<String, String> {
        SeedJwtVar::var("SEED_JWT")
    }
}
