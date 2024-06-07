use crate::domain::models::cash_in::CashIn;
use crate::domain::repositories::cash_in::CashInRepository;
use crate::infrastructure::databases::postgresql::DBConn;
use async_trait::async_trait;
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
    async fn create(&self, data: CashIn) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn read(&self, id: i32) -> Option<CashIn> {
        None
    }
    async fn update(&self, id: i32, data: CashIn) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
