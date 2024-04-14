use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Pair {
    pub initial: String,
    pub next: String,
    pub object: String,
    pub image: String,
}

#[derive(Debug, Deserialize)]
pub enum LoginStatus {
    Success,
    Failed,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub status: LoginStatus,
    pub username: String,
}
