use super::{Service, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Appointment<'a> {
    id: Uuid,

    #[serde(borrow)]
    client: User<'a>,
    service: Service<'a>,

    date_start: DateTime<Utc>,
    date_end: DateTime<Utc>,
    status: AppointmentStatus,
    observations: &'a str,
}

#[derive(Serialize, Deserialize)]
pub enum AppointmentStatus {
    Pending,
    Confirmed,
    Completed,
    CanceledClient,
    CancelledAdmin,
}
