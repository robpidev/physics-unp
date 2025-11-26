use super::Vars;
use crate::set_vars;

/// Struct for server config
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            host: String::from("127.0.0.1"),
            port: 8080,
        }
    }
}

impl Vars for Server {}

impl Server {
    pub fn from_env() -> Self {
        let mut server = Self::default();

        set_vars!(server.host, "HOST");
        set_vars!(server.port, "PORT");

        server
    }
}
