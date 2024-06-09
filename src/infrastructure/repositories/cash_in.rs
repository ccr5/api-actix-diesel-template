use crate::domain::models::cash_in::{CashIn, NewCashIn};
use crate::domain::repositories::cash_in::CashInRepository;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::cash_in::{CashInDiesel, NewCashInDiesel};
use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;
use std::sync::Arc;

pub struct CashInRepositoryImpl {
    pub pool: Arc<DBConn>,
}

impl CashInRepositoryImpl {
    pub fn new(db: Arc<DBConn>) -> Self {
        CashInRepositoryImpl { pool: db }
    }
}

#[async_trait]
impl CashInRepository for CashInRepositoryImpl {
    async fn create(&self, data: NewCashIn) -> Result<CashIn, Box<dyn Error>> {
        use crate::schema::cash_in::dsl::cash_in;
        let new_cash_in_diesel = NewCashInDiesel::from(data.clone());
        let mut conn = self.pool.get().unwrap();
        let result: CashInDiesel = run(move || {
            diesel::insert_into(cash_in)
                .values(new_cash_in_diesel)
                .get_result(&mut conn)
        })
        .await
        .unwrap();
        Ok(result.into())
    }

    async fn read(&self, cash_id: i32) -> Option<CashIn> {
        use crate::schema::cash_in::dsl::cash_in;
        let mut conn = self.pool.get().unwrap();
        let res: CashInDiesel = run(move || cash_in.find(cash_id).first::<CashInDiesel>(&mut conn))
            .await
            .unwrap();
        Some(res.into())
    }

    async fn delete(&self, cash_id: i32) -> Result<(), Box<dyn Error>> {
        use crate::schema::cash_in::dsl::{cash_in, id};
        let mut conn = self.pool.get().unwrap();
        run(move || {
            diesel::delete(cash_in)
                .filter(id.eq(cash_id))
                .execute(&mut conn)
        })
        .await
        .unwrap();
        Ok(())
    }
}
