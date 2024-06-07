pub enum CashOutType {
    Pix,
    CreditCard,
    DebitCard,
    Money,
}

pub struct CashOut {
    id: i32,
    amount: f32,
    description: String,
    cash_out_type: CashOutType,
    installment_number: i32,
    total_installments: i32,
    date: String,
    status: String,
    observation: String,
}
