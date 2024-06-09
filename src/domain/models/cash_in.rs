use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CashIn {
    pub id: i32,
    pub description: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewCashIn {
    pub description: String,
    pub status: String,
}
