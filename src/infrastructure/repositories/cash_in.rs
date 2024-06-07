use crate::domain::models::cash_in::CashIn;
use crate::domain::repositories::cash_in::CashInRepository;
use async_trait::async_trait;
use std::error::Error;

pub struct CashInRepositoryImpl;

impl CashInRepositoryImpl {
    pub fn new() -> Self {
        CashInRepositoryImpl
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
