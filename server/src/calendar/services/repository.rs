use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::shared::repository::db;

pub type DB = db::DB;

#[derive(Deserialize)]
struct ScheduleDB {
    id: Thing,
    todo: String,
    from: String,
    to: String,
}

#[derive(Serialize)]
struct Schedule {
    id: String,
    todo: String,
    from: String,
    to: String,
}

pub async fn get_datetimes(db: &DB) -> Result<impl Serialize, (u16, String)> {
    let query = "SELECT for AS todo, from, to, id FROM register_time;";

    let mut result = match db.query(query).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB contion error: {}", e.to_string()))),
    };

    let schedules = match result.take::<Vec<ScheduleDB>>(0) {
        Ok(s) => s,
        Err(e) => return Err((500, format!("DB parse error: {}", e.to_string()))),
    };

    Ok(schedules
        .into_iter()
        .map(|s| Schedule {
            id: s.id.id.to_string(),
            todo: s.todo,
            from: s.from,
            to: s.to,
        })
        .collect::<Vec<Schedule>>())
}

pub async fn delete(id: &String, db: &DB) -> Result<(), (u16, String)> {
    let query = r#"DELETE type::thing("register_time", <string>$id)"#;

    let mut resp = match db.query(query).bind(("id", id)).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB contion error: {}", e.to_string()))),
    };

    match resp.take::<Vec<()>>(0) {
        Ok(_) => Ok(()),
        Err(e) => Err((500, format!("DB parse error: {}", e.to_string()))),
    }
}
