use super::{Log, Roles};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User<'a> {
    id: Uuid,
    full_name: &'a str,
    email: &'a str,
    cpf: &'a str,
    phone: &'a str,
    birth: DateTime<Utc>,
    role: Roles,
    active: bool,
    log: Log,
}
