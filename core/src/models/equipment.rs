use super::Log;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Equipment<'a> {
    id: Uuid,
    name: &'a str,
    serial: &'a str,
    status: EquipmentStatus,
    purchased_at: DateTime<Utc>,
    log: Log,
}

#[derive(Serialize, Deserialize)]
pub enum EquipmentStatus {
    Operational,
    Broken,
    Missing,
    Maintenance,
}
