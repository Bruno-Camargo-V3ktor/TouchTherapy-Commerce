use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Roles {
    Client,
    Employee,
    Admin,
}
