use dotenv::dotenv;
use std::env;

pub mod auth;
pub mod calendar;
pub mod course;
pub mod evaluation;
pub mod faculty;
pub mod professor;
pub mod school;
pub mod shared;
pub mod student;

pub struct AppData {
    pub host: String,
    pub db_host: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_use_ns: String,
    pub db_use_db: String,
}

pub fn app_config() -> AppData {
    dotenv().ok();
    AppData {
        host: match env::var("HOST") {
            Ok(host) => host,
            Err(_) => {
                println!("HOST not set, using 0.0.0.0:8080");
                String::from("0.0.0.0:8080")
            }
        },
        db_host: match env::var("DB_HOST") {
            Ok(host) => host,
            Err(_) => {
                println!("DB_HOST not set, using 0.0.0.0:8000");
                String::from("0.0.0.0:8000")
            }
        },

        db_user: match env::var("DB_USER") {
            Ok(user) => user,
            Err(_) => {
                println!("DB_USER not set, using root");
                String::from("root")
            }
        },

        db_pass: match env::var("DB_PASS") {
            Ok(pass) => pass,
            Err(_) => {
                println!("DB_PASS not set, using root");
                String::from("root")
            }
        },

        db_use_ns: match env::var("DB_USE_NS") {
            Ok(ns) => ns,
            Err(_) => {
                println!("DB_USE_NS not set, using test");
                String::from("test")
            }
        },

        db_use_db: match env::var("DB_USE_DB") {
            Ok(db) => db,
            Err(_) => {
                println!("DB_USE_DB not set, using test");
                String::from("test")
            }
        },
    }
}
