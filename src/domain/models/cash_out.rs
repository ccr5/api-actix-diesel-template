use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CashOutType {
    Pix,
    CreditCard,
    DebitCard,
    Money,
}

#[derive(Serialize, Deserialize)]
pub struct CashOut {
    pub id: i32,
    pub amount: f32,
    pub description: String,
    pub cash_out_type: CashOutType,
    pub installment_number: i32,
    pub total_installments: i32,
    pub date: String,
    pub status: String,
    pub observation: String,
}
