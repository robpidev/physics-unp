#[derive(Debug, Clone)]
pub struct LastName {
    last_name: String,
}

impl ToString for LastName {
    fn to_string(&self) -> String {
        self.last_name.clone()
    }
}

impl LastName {
    pub fn new(last_name: String) -> Result<Self, String> {
        let last_name = last_name.trim() as &str;

        if last_name.is_empty() {
            return Err("Last name cannot be empty".to_string());
        }

        if last_name.chars().any(|c| !c.is_alphabetic()) {
            return Err("Last name must only contain alphabetic characters".to_string());
        }

        let last_name = last_name[0..1].to_uppercase() + &last_name[1..].to_lowercase();

        Ok(Self { last_name })
    }
}

#[test]
fn test_last_name() {
    let last_name = LastName::new(" torres ".to_string()).unwrap();
    assert_eq!(last_name.last_name, "Torres");

    let last_name = LastName::new("  ".to_string());
    assert!(last_name.is_err());

    let last_name = LastName::new("torre1s".to_string());
    assert!(last_name.is_err());

    let last_name = LastName::new("torres torres".to_string());
    assert!(last_name.is_err());

    let last_name = LastName::new("t orres".to_string());
    assert!(last_name.is_err());
}
