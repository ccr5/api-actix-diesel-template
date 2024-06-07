use crate::domain::services::cash_in::CashInService;
use crate::domain::services::cash_out::CashOutService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::cash_in::CashInRepositoryImpl;
use crate::infrastructure::repositories::cash_out::CashOutRepositoryImpl;
use crate::services::cash_in::CashInServiceImpl;
use crate::services::cash_out::CashOutServiceImpl;
use std::sync::Arc;

#[derive(Clone)]
pub struct Container {
    pub cash_in_service: Arc<dyn CashInService>,
    pub cash_out_service: Arc<dyn CashOutService>,
}

impl Container {
    pub fn new() -> Self {
        let cash_in_repository = Arc::new(CashInRepositoryImpl::new(Arc::new(db_pool())));
        let cash_in_service = Arc::new(CashInServiceImpl {
            repository: cash_in_repository,
        });
        let cash_out_repository = Arc::new(CashOutRepositoryImpl::new(Arc::new(db_pool())));
        let cash_out_service = Arc::new(CashOutServiceImpl {
            repository: cash_out_repository,
        });

        Self {
            cash_in_service,
            cash_out_service,
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
