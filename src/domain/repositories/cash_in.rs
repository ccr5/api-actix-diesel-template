use crate::domain::models::cash_in::{CashIn, NewCashIn};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait CashInRepository: Send + Sync {
    async fn create(&self, data: NewCashIn) -> Result<CashIn, Box<dyn Error>>;
    async fn read(&self, cash_id: i32) -> Option<CashIn>;
    async fn delete(&self, cash_id: i32) -> Result<(), Box<dyn Error>>;
}
