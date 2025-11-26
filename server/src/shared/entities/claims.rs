use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims<T> {
    pub data: T,
    pub exp: usize,
}
