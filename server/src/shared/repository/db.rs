use crate::config::db::DBVars;
use std::sync::LazyLock;
use surrealdb::{
    engine::{
        // any::Any,
        remote::ws::{Client, Ws},
    },
    opt::auth::Root,
    Surreal,
};

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub struct DBInstance;

impl DBInstance {
    pub async fn db_connect() -> Result<(), String> {
        println!("Connecting to DB...");
        let config = match DBVars::from_env() {
            Ok(config) => config,
            Err(e) => return Err(e),
        };

        println!("config: {:#?}", config);

        match DB.connect::<Ws>(config.ws).await {
            Ok(_) => println!("Conected to DB"),
            Err(e) => return Err(format!("DB connect error: {}", e)),
        };

        let root = Root {
            username: &config.user,
            password: &config.password,
        };

        match DB.signin(root).await {
            Ok(_) => println!("Signed in"),
            Err(e) => return Err(format!("DB signin error: {}", e)),
        }

        match DB.use_ns(&config.name_space).use_db(&config.name).await {
            Ok(_) => println!("DB ns: {}; db: {}", config.name_space, config.name),
            Err(e) => return Err(format!("DB ns error: {}", e)),
        }

        println!("DB connected");

        Ok(())
    }
}
