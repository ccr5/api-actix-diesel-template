use crate::domain::models::cash_out::{CashOut, NewCashOut};
use crate::domain::repositories::cash_out::CashOutRepository;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::cash_out::{CashOutDiesel, NewCashOutDiesel};
use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;
use std::sync::Arc;

pub struct CashOutRepositoryImpl {
    pub pool: Arc<DBConn>,
}

impl CashOutRepositoryImpl {
    pub fn new(db: Arc<DBConn>) -> Self {
        CashOutRepositoryImpl { pool: db }
    }
}

#[async_trait]
impl CashOutRepository for CashOutRepositoryImpl {
    async fn create(&self, data: NewCashOut) -> Result<(), Box<dyn Error>> {
        use crate::schema::cash_out::dsl::cash_out;
        let new_cash_out_diesel = NewCashOutDiesel::from(data.clone());
        let mut conn = self.pool.get().unwrap();
        let _result: CashOutDiesel = run(move || {
            diesel::insert_into(cash_out)
                .values(new_cash_out_diesel)
                .get_result(&mut conn)
        })
        .await
        .unwrap();
        Ok(())
    }

    async fn read(&self, cash_id: i32) -> Option<CashOut> {
        use crate::schema::cash_out::dsl::cash_out;
        let mut conn = self.pool.get().unwrap();
        let res: CashOutDiesel =
            run(move || cash_out.find(cash_id).first::<CashOutDiesel>(&mut conn))
                .await
                .unwrap();
        Some(res.into())
    }

    async fn delete(&self, cash_id: i32) -> Result<(), Box<dyn Error>> {
        use crate::schema::cash_out::dsl::{cash_out, id};
        let mut conn = self.pool.get().unwrap();
        run(move || {
            diesel::delete(cash_out)
                .filter(id.eq(cash_id))
                .execute(&mut conn)
        })
        .await
        .unwrap();
        Ok(())
    }
}
