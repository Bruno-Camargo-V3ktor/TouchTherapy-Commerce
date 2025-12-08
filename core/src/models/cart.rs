use super::{Log, Product, Service, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Cart<'a> {
    pub id: Uuid,
    pub user: Option<User<'a>>,
    pub session_token: &'a str,
    pub products: Vec<(Product<'a>, u16)>,
    pub services: Vec<(Service<'a>, DateTime<Utc>)>,
    pub log: Log,
}
