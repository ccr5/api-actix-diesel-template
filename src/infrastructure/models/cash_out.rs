use crate::domain::models::cash_out::CashOut;
use crate::infrastructure::schema::cash_out;
use diesel;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = cash_out)]
pub struct CashOutDiesel {
    pub id: i32,
    pub amount: f32,
    pub description: String,
    pub cash_out_type: String,
    pub installment_number: i32,
    pub total_installments: i32,
    pub date: String,
    pub status: String,
    pub observation: String,
}

impl From<CashOut> for CashOutDiesel {
    fn from(t: CashOut) -> Self {
        CashOutDiesel {
            id: t.id,
            amount: t.amount,
            description: t.description,
            cash_out_type: match t.cash_out_type {
                CashOutType::Pix => "Pix".to_string(),
                CashOutType::CreditCard => "CreditCard".to_string(),
                CashOutType::DebitCard => "DebitCard".to_string(),
                CashOutType::Money => "Money".to_string(),
            },
            installment_number: t.installment_number,
            total_installments: t.total_installments,
            date: t.date,
            status: t.status,
            observation: t.observation,
        }
    }
}

impl Into<CashOut> for CashOutDiesel {
    fn into(self) -> CashOut {
        CashOut {
            id: self.id,
            amount: self.amount,
            description: self.description,
            cash_out_type: match self.cash_out_type.as_str() {
                "Pix" => CashOutType::Pix,
                "CreditCard" => CashOutType::CreditCard,
                "DebitCard" => CashOutType::DebitCard,
                "Money" => CashOutType::Money,
                _ => CashOutType::Money,
            },
            installment_number: self.installment_number,
            total_installments: self.total_installments,
            date: self.date,
            status: self.status,
            observation: self.observation,
        }
    }
}
