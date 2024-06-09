use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum CashOutType {
    Pix,
    CreditCard,
    DebitCard,
    Money,
}

#[derive(Serialize, Deserialize)]
pub struct CashOut {
    pub id: i32,
    pub description: String,
    pub installment_number: Option<i32>,
    pub total_installments: Option<i32>,
    pub status: String,
    pub observation: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewCashOut {
    pub description: String,
    pub installment_number: Option<i32>,
    pub total_installments: Option<i32>,
    pub status: String,
    pub observation: Option<String>,
}
