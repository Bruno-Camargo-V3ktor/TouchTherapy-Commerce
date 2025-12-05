mod equipment;
mod roles;
mod service;
mod user;

use chrono::{DateTime, Utc};
pub use equipment::*;
pub use roles::*;
use serde::{Deserialize, Serialize};
pub use service::*;
pub use user::*;

#[derive(Serialize, Deserialize)]
pub struct Log {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
