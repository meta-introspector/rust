use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitErrata {
    pub oid: String,
    pub error_message: String,
    pub timestamp: DateTime<Utc>,
}

impl GitErrata {
    pub fn new(oid: String, error_message: String) -> Self {
        GitErrata {
            oid,
            error_message,
            timestamp: Utc::now(),
        }
    }
}
