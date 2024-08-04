use serde::Serialize;

#[derive(Serialize)]
pub struct Names {
    names: String,
}

impl ToString for Names {
    fn to_string(&self) -> String {
        self.names.clone()
    }
}

impl Names {
    pub fn new(names: String) -> Result<Self, String> {
        let names = names.trim();

        if names.is_empty() {
            return Err("Names cannot be empty".to_string());
        }

        let name_formated = names
            .split_whitespace()
            .map(|name| {
                if name.chars().all(|c| c.is_alphabetic()) {
                    Ok(name[0..1].to_uppercase() + name[1..].to_lowercase().as_str())
                } else {
                    return Err("Names must only contain alphabetic characters".to_string());
                }
            })
            .collect::<Result<Vec<String>, String>>()?
            .join(" ");

        Ok(Self {
            names: name_formated,
        })
    }
}

#[test]
fn test_names() {
    let names = Names::new(" rober  Esbel ".to_string()).unwrap();
    assert_eq!(names.names, "Rober Esbel");

    let names = Names::new("  ".to_string());
    assert!(names.is_err());

    let names = Names::new("rob1er esbel".to_string());
    assert!(names.is_err());

    let names = Names::new("dAnieala   ElizaBeth   FErnandez  ".to_string());
    assert_eq!(names.unwrap().names, "Danieala Elizabeth Fernandez");
}