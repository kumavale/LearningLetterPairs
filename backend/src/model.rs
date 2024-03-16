use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

use crate::auth::EXP_DAYS;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: u64,
    pub name: String,
    exp: i64,
}

impl Claims {
    pub fn new(id: u64, name: String) -> Self {
        let exp = (Utc::now() + Duration::try_days(EXP_DAYS).unwrap()).timestamp();
        Self { id, name, exp, }
    }
}
