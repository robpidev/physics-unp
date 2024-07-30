use serde::Serialize;

#[derive(Serialize)]
pub struct Evaluation {
    course: String,
    etype: String,
    score: u8,
    number: u8,
}
