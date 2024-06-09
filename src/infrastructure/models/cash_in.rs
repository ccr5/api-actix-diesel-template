use crate::domain::models::cash_in::{CashIn, NewCashIn};
use crate::schema::cash_in;
use diesel;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct CashInDiesel {
    pub id: i32,
    pub description: String,
    pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = cash_in)]
pub struct NewCashInDiesel {
    pub description: String,
    pub status: String,
}

impl Into<CashIn> for CashInDiesel {
    fn into(self) -> CashIn {
        CashIn {
            id: self.id,
            description: self.description,
            status: self.status,
        }
    }
}

impl From<NewCashIn> for NewCashInDiesel {
    fn from(t: NewCashIn) -> Self {
        NewCashInDiesel {
            description: t.description,
            status: t.status,
        }
    }
}

impl Into<CashIn> for NewCashInDiesel {
    fn into(self) -> CashIn {
        CashIn {
            id: 0,
            description: self.description,
            status: self.status,
        }
    }
}
