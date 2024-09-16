use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::shared::repository::db;

pub type DB = db::DB;

#[derive(Deserialize)]
struct ScheduleDB {
    id: Thing,
    todo: String,
    start: String,
    end: String,
}

#[derive(Serialize)]
struct Schedule {
    id: String,
    todo: String,
    start: String,
    end: String,
}

impl ScheduleDB {
    fn to_schedule(&self) -> Schedule {
        Schedule {
            id: self.id.id.to_string(),
            todo: self.todo.clone(),
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }
}

pub async fn get_datetimes(db: &DB) -> Result<impl Serialize, (u16, String)> {
    let query = "SELECT todo, start, end, id FROM register_time;";

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
        .map(|s| s.to_schedule())
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

pub async fn add(todo: &String, end: &String, db: &DB) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
    CREATE register_time
    SET todo = <string>$todo,
    end = <datetime>$end;
    "#;

    let mut resp = match db
        .query(query)
        .bind(("todo", todo))
        .bind(("end", end))
        .await
    {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB contion error: {}", e.to_string()))),
    };

    let sc = match resp.take::<Option<ScheduleDB>>(0) {
        Ok(s) => s,
        Err(e) => return Err((500, format!("DB response error: {}", e.to_string()))),
    };

    match sc {
        Some(s) => return Ok(s.to_schedule()),
        None => Err((500, "DB response None".to_string())),
    }
}
