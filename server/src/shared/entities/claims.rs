use serde::Serialize;

#[derive(Serialize)]
pub struct Claims<T> {
    pub data: T,
    pub exp: usize,
}
