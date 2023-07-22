use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: u64,
    pub name: String,
    pub exp: usize,
}
