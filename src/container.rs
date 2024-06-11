use crate::domain::services::cash_in::CashInService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::cash_in::CashInRepositoryImpl;
use crate::services::cash_in::CashInServiceImpl;
use std::sync::Arc;

#[derive(Clone)]
pub struct Container {
    pub cash_in_service: Arc<dyn CashInService>,
}

impl Container {
    pub fn new() -> Self {
        let cash_in_repository = Arc::new(CashInRepositoryImpl::new(Arc::new(db_pool())));
        let cash_in_service = Arc::new(CashInServiceImpl {
            repository: cash_in_repository,
        });

        Self { cash_in_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
