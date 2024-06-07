use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CashIn {
    id: i32,
    amount: f32,
    description: String,
    date: String,
    status: String,
}
