use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementData {
    pub measurement_db_id: Uuid,
    pub ts: DateTime<FixedOffset>,
    pub index: i64,
    pub rssi: f64,
    pub equipment_db_id: Uuid,
    pub parameter_db_id: Uuid,
    pub parameter_type_db_id: Uuid,
    pub sensor_db_id: Uuid,
    pub value: f64,
}
