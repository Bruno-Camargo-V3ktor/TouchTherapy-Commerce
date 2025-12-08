use super::{Log, PaymentMethod, Product, User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Order<'a> {
    pub id: Uuid,

    pub user: User<'a>,
    pub producuts: Vec<ProductInfo<'a>>,

    pub status: OrderStatus,
    pub payment: PaymentMethod,
    pub total: f64,
    pub nfe: &'a str,

    pub log: Log,
}

#[derive(Serialize, Deserialize)]
pub struct ProductInfo<'a> {
    #[serde(borrow)]
    pub product: Product<'a>,
    pub quantity: u16,
    pub price: f64,
}

#[derive(Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Paid,
    Sent,
    Delivered,
    Canceled,
    Reversed,
    Completed,
}
