use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum PaymentMethod {
    Pix,
    CreditcCard,
    DebitCard,
    Boleto,
}

#[derive(Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Paid,
    Canceled,
    Reversed,
}
