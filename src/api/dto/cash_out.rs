use crate::domain::models::cash_out::{CashOut, NewCashOut};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CashOutDTO {
    id: i32,
    description: String,
    installment_number: Option<i32>,
    total_installments: Option<i32>,
    status: String,
    observation: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewCashOutDTO {
    description: String,
    installment_number: Option<i32>,
    total_installments: Option<i32>,
    status: String,
    observation: Option<String>,
}

impl Into<CashOut> for CashOutDTO {
    fn into(self) -> CashOut {
        CashOut {
            id: self.id,
            description: self.description,
            installment_number: self.installment_number,
            total_installments: self.total_installments,
            status: self.status,
            observation: self.observation,
        }
    }
}

impl Into<CashOutDTO> for CashOut {
    fn into(self) -> CashOutDTO {
        CashOutDTO {
            id: self.id,
            description: self.description,
            installment_number: self.installment_number,
            total_installments: self.total_installments,
            status: self.status,
            observation: self.observation,
        }
    }
}

impl Into<NewCashOut> for NewCashOutDTO {
    fn into(self) -> NewCashOut {
        NewCashOut {
            description: self.description,
            installment_number: self.installment_number,
            total_installments: self.total_installments,
            status: self.status,
            observation: self.observation,
        }
    }
}

impl Into<NewCashOutDTO> for NewCashOut {
    fn into(self) -> NewCashOutDTO {
        NewCashOutDTO {
            description: self.description,
            installment_number: self.installment_number,
            total_installments: self.total_installments,
            status: self.status,
            observation: self.observation,
        }
    }
}
