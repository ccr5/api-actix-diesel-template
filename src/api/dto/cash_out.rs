use crate::domain::models::cash_out::{CashOut, CashOutType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CashOutDTO {
    id: i32,
    amount: f32,
    description: String,
    cash_out_type: CashOutType,
    installment_number: i32,
    total_installments: i32,
    date: String,
    status: String,
    observation: String,
}

impl Into<CashOut> for CashOutDTO {
    fn into(self) -> CashOut {
        CashOut {
            id: self.id,
            amount: self.amount,
            description: self.description,
            cash_out_type: self.cash_out_type,
            installment_number: self.installment_number,
            total_installments: self.total_installments,
            date: self.date,
            status: self.status,
            observation: self.observation,
        }
    }
}

impl Into<CashOutDTO> for CashOut {
    fn into(self) -> CashOutDTO {
        CashOutDTO {
            id: self.id,
            amount: self.amount,
            description: self.description,
            cash_out_type: self.cash_out_type,
            installment_number: self.installment_number,
            total_installments: self.total_installments,
            date: self.date,
            status: self.status,
            observation: self.observation,
        }
    }
}
