use crate::domain::models::cash_in::CashIn;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CashInDTO {
    id: i32,
    amount: f32,
    description: String,
    date: String,
    status: String,
}

impl Into<CashIn> for CashInDTO {
    fn into(self) -> CashIn {
        CashIn {
            id: self.id,
            amount: self.amount,
            description: self.description,
            date: self.date,
            status: self.status,
        }
    }
}

impl Into<CashInDTO> for CashIn {
    fn into(self) -> CashInDTO {
        CashInDTO {
            id: self.id,
            amount: self.amount,
            description: self.description,
            date: self.date,
            status: self.status,
        }
    }
}
