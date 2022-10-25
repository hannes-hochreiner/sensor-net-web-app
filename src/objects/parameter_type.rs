use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ParameterType {
    pub db_id: Uuid,
    pub db_rev: Uuid,
    pub id: String,
    pub unit: String,
    pub info: Option<String>,
}
