use es_interface::*;

#[derive(Debug, Clone)]
pub struct UserId {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct PaymentId {
    pub id: String,
}

/// Event demonstrating field paths for idempotency and correlation
#[derive(Debug, Clone, es_derive2::InjectableEvent)]
#[es(idempotency = ["payment_id"], correlation = ["user_id"])]
pub struct PaymentProcessed {
    pub payment: PaymentId,
    pub user: UserId,
    pub amount: u64,
    pub payment_id: String,
    pub user_id: String,
}
