use crate::domain::models::cash_in::CashIn;
use crate::infrastructure::schema::cash_in;
use diesel;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = cash_in)]
pub struct CashInDiesel {
    pub id: i32,
    pub amount: f32,
    pub description: String,
    pub date: String,
    pub status: String,
}

impl From<CashIn> for CashInDiesel {
    fn from(t: CashIn) -> Self {
        CashInDiesel {
            id: t.id,
            amount: t.amount,
            description: t.description,
            date: t.date,
            status: t.status,
        }
    }
}

impl Into<CashIn> for CashInDiesel {
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
