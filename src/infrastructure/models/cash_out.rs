use crate::domain::models::cash_out::{CashOut, NewCashOut};
use crate::schema::cash_out;
use diesel;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct CashOutDiesel {
    pub id: i32,
    pub description: String,
    pub installment_number: Option<i32>,
    pub total_installments: Option<i32>,
    pub status: String,
    pub observation: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = cash_out)]
pub struct NewCashOutDiesel {
    pub description: String,
    pub installment_number: Option<i32>,
    pub total_installments: Option<i32>,
    pub status: String,
    pub observation: Option<String>,
}

impl Into<CashOut> for CashOutDiesel {
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

impl From<NewCashOut> for NewCashOutDiesel {
    fn from(t: NewCashOut) -> Self {
        NewCashOutDiesel {
            description: t.description,
            installment_number: t.installment_number,
            total_installments: t.total_installments,
            status: t.status,
            observation: t.observation,
        }
    }
}

impl Into<CashOut> for NewCashOutDiesel {
    fn into(self) -> CashOut {
        CashOut {
            id: 0,
            description: self.description,
            installment_number: self.installment_number,
            total_installments: self.total_installments,
            status: self.status,
            observation: self.observation,
        }
    }
}
