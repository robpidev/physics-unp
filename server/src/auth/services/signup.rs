use crate::auth::entities::professor::Professor;

pub fn signup_professor(
    names: String,
    last_name1: String,
    last_name2: String,
    dni: String,
) -> Result<String, (u16, String)> {
    let professor = match Professor::new(names, last_name1, last_name2, dni) {
        Ok(p) => p,
        Err(e) => return Err((400u16, e)),
    };

    Ok(professor.to_json())
}
