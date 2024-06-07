use crate::domain::models::cash_out::CashOut;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait CashOutRepository: Send + Sync {
    async fn create(&self, data: CashOut) -> Result<(), Box<dyn Error>>;
    async fn read(&self, id: i32) -> Option<CashOut>;
    async fn update(&self, id: i32, data: CashOut) -> Result<(), Box<dyn Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}
