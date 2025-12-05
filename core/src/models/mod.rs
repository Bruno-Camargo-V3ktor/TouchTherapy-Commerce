use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod appointment;
mod equipment;
mod roles;
mod service;
mod user;

pub use appointment::*;
pub use equipment::*;
pub use roles::*;
pub use service::*;
pub use user::*;

#[derive(Serialize, Deserialize)]
pub struct Log {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
