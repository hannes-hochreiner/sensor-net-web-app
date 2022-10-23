use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sensor {
    pub db_id: Uuid,
    pub db_rev: Uuid,
    pub id: String,
    pub info: Option<String>,
}
