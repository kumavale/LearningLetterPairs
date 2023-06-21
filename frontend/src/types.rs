use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Pair {
    pub initial: String,
    pub next: String,
    pub object: String,
    pub image: String,
}

