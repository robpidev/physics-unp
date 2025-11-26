use std::{fmt::Display, str::FromStr};

pub mod db;
mod seed;
mod server;

pub use seed::SeedJwtVar;
pub use server::Server;

/// Macros
#[macro_export]
macro_rules! set_vars {
    ($field:expr, $env:expr) => {
        match Self::var::<_>($env) {
            Ok(val) => $field = val,
            Err(e) => println!("Error: {}; Default: {}", e, $field),
        }
    };
}

/// Load .env file
pub fn load_env() {
    if let Err(e) = dotenvy::dotenv() {
        panic!("Failed to load .env: {}", e);
    } else {
        println!("Loaded .env");
    };
}

/// Get environment variable type: ```NAME=VALUE```
pub trait Vars {
    fn var<T>(name: &str) -> Result<T, String>
    where
        T: FromStr,
        T::Err: Display,
    {
        let val = match std::env::var(name) {
            Ok(val) => val,
            Err(e) => return Err(format!("{}: {}", name, e)),
        };

        val.parse().map_err(|e| format!("{}: {}", name, e))
    }
}
