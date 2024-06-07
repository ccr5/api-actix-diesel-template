use crate::adapters::repositories::cash_in_repository::CashInRepositoryImpl;
use crate::services::cash_flow::cash_in_usecases::{CashInUseCases, CashInUseCasesImpl};
use std::sync::Arc;

#[derive(Clone)]
pub struct Container {
    pub cash_in_usecases: Arc<dyn CashInUseCases>,
}

impl Container {
    pub fn new() -> Self {
        let cash_in_repository = Arc::new(CashInRepositoryImpl);
        let cash_in_usecases = Arc::new(CashInUseCasesImpl { cash_in_repository });
        Self { cash_in_usecases }
    }
}
