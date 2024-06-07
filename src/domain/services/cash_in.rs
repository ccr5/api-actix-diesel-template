use crate::domain::models::cash_in::CashIn;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait CashInService: Send + Sync {
    async fn create(&self, data: CashIn) -> Result<(), Box<dyn Error>>;
    async fn read(&self, id: i32) -> Option<CashIn>;
    async fn update(&self, id: i32, data: CashIn) -> Result<(), Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
