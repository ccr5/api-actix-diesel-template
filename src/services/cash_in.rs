use crate::domain::models::cash_in::{CashIn, NewCashIn};
use crate::domain::repositories::cash_in::CashInRepository;
use crate::domain::services::cash_in::CashInService;
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct CashInServiceImpl {
    pub repository: Arc<dyn CashInRepository>,
}

impl CashInServiceImpl {
    pub fn new(repository: Arc<dyn CashInRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CashInService for CashInServiceImpl {
    async fn create(&self, data: NewCashIn) -> Result<CashIn, Box<dyn Error>> {
        self.repository.create(data).await
    }

    async fn read(&self, cash_id: i32) -> Option<CashIn> {
        self.repository.read(cash_id).await
    }

    async fn delete(&self, cash_id: i32) -> Result<(), Box<dyn Error>> {
        self.repository.delete(cash_id).await
    }
}
