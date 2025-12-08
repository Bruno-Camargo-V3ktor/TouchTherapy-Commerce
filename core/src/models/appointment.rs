use super::{PaymentMethod, PaymentStatus, Service, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Appointment<'a> {
    pub id: Uuid,

    pub client: User<'a>,
    pub professional: User<'a>,
    pub service: Service<'a>,

    pub date_start: Vec<DateTime<Utc>>,
    pub date_end: Vec<DateTime<Utc>>,
    pub status: AppointmentStatus,
    pub payment_method: PaymentMethod,
    pub payment_status: PaymentStatus,

    pub nfe: &'a str,
    pub observations: &'a str,
}

#[derive(Serialize, Deserialize)]
pub enum AppointmentStatus {
    Pending,
    Paid,
    Confirmed,
    Completed,
    Canceled,
}
