use es_core::*;

#[derive(Debug, Clone)]
pub struct UserId {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct PaymentId {
    pub id: String,
}

/// Event demonstrating direct field access (simpler test first)
#[derive(Debug, Clone, es_derive2::Event)]
pub struct PaymentProcessed {
    pub payment: PaymentId,
    pub user: UserId,
    pub amount: u64,

    // Direct field idempotency
    #[es(idempotency)]
    pub payment_id: String,

    // Direct field correlation with explicit status
    #[es(correlation)]
    pub user_id: String,
}
