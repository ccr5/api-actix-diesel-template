use crate::domain::models::cash_out::{CashOut, NewCashOut};
use crate::domain::repositories::cash_out::CashOutRepository;
use crate::domain::services::cash_out::CashOutService;
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct CashOutServiceImpl {
    pub repository: Arc<dyn CashOutRepository>,
}

impl CashOutServiceImpl {
    pub fn new(repository: Arc<dyn CashOutRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CashOutService for CashOutServiceImpl {
    async fn create(&self, data: NewCashOut) -> Result<(), Box<dyn Error>> {
        self.repository.create(data).await
    }

    async fn read(&self, cash_id: i32) -> Option<CashOut> {
        self.repository.read(cash_id).await
    }

    async fn delete(&self, cash_id: i32) -> Result<(), Box<dyn Error>> {
        self.repository.delete(cash_id).await
    }
}
