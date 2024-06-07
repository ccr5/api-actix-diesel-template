use crate::domain::models::cash_in::CashIn;
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
    async fn create(&self, data: CashIn) -> Result<(), Box<dyn Error>> {
        self.repository.create(data).await
    }

    async fn read(&self, id: i32) -> Option<CashIn> {
        self.repository.read(id).await
    }

    async fn update(&self, id: i32, data: CashIn) -> Result<(), Box<dyn Error>> {
        self.repository.update(id, data).await
    }

    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        self.repository.delete(id).await
    }
}
