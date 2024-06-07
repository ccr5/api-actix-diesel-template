use crate::domain::models::cash_out::CashOut;
use crate::domain::repositories::cash_out::CashOutRepository;
use crate::infrastructure::databases::postgresql::DBConn;
use async_trait::async_trait;
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
    async fn create(&self, data: CashOut) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn read(&self, id: i32) -> Option<CashOut> {
        None
    }
    async fn update(&self, id: i32, data: CashOut) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
