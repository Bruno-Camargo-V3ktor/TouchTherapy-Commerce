use super::Equipment;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Service<'a> {
    id: Uuid,
    name: &'a str,
    description: &'a str,
    duration: u8,
    price: f64,
    equipments: Vec<Equipment>,
    active: bool,
}
