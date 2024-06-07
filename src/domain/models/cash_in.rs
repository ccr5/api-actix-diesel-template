use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CashIn {
    pub id: i32,
    pub amount: f32,
    pub description: String,
    pub date: String,
    pub status: String,
}
