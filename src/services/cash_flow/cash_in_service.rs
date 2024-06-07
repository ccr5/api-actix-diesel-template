use crate::adapters::repositories::cash_in_repository::CashInRepository;
use crate::entities::cash_flow::cash_in::CashIn;
use std::error::Error;
use std::sync::Arc;

pub trait CashInUseCases: Send + Sync {
    // Create a new cash_in entry
    fn create(&self, data: CashIn) -> Result<(), Box<dyn Error>>;

    // Read a cash_in entry by id
    fn read(&self, id: i32) -> Option<CashIn>;

    // Update a cash_in entry by id
    fn update(&self, id: i32, data: CashIn) -> Result<(), Box<dyn Error>>;

    // Delete a cash_in entry by id
    fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}

pub struct CashInUseCasesImpl {
    pub cash_in_repository: Arc<dyn CashInRepository>,
}

impl CashInUseCases for CashInUseCasesImpl {
    fn create(&self, data: CashIn) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn read(&self, id: i32) -> Option<CashIn> {
        Some(CashIn::new(
            id,
            0.0,
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ))
    }

    fn update(&self, id: i32, data: CashIn) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
