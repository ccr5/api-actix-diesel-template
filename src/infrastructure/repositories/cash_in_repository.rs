use crate::entities::cash_flow::cash_in::CashIn;
use std::error::Error;

pub trait CashInRepository: Send + Sync {
    fn create(&self, data: CashIn) -> Result<(), Box<dyn Error>>;

    fn read(&self, id: i32) -> Option<CashIn>;

    fn update(&self, id: i32, data: CashIn) -> Result<(), Box<dyn Error>>;

    fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
}

pub struct CashInRepositoryImpl;

impl CashInRepository for CashInRepositoryImpl {
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
