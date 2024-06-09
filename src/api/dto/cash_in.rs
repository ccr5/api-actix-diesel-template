use crate::domain::models::cash_in::{CashIn, NewCashIn};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CashInDTO {
    id: i32,
    description: String,
    status: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewCashInDTO {
    description: String,
    status: String,
}

impl Into<CashIn> for CashInDTO {
    fn into(self) -> CashIn {
        CashIn {
            id: self.id,
            description: self.description,
            status: self.status,
        }
    }
}

impl Into<CashInDTO> for CashIn {
    fn into(self) -> CashInDTO {
        CashInDTO {
            id: self.id,
            description: self.description,
            status: self.status,
        }
    }
}

impl Into<NewCashIn> for NewCashInDTO {
    fn into(self) -> NewCashIn {
        NewCashIn {
            description: self.description,
            status: self.status,
        }
    }
}

impl Into<NewCashInDTO> for NewCashIn {
    fn into(self) -> NewCashInDTO {
        NewCashInDTO {
            description: self.description,
            status: self.status,
        }
    }
}
