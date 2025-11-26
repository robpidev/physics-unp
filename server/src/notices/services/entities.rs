use serde::Serialize;

#[derive(Serialize)]
pub struct Notice {
    pub id: String,
    pub note: String,
    pub datetime: String,
}
