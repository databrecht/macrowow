use es_core::*;

#[derive(Debug, Clone)]
pub struct UserId {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct PaymentId {
    pub id: String,
}

/// Event demonstrating dotted path access for idempotency and correlation
#[derive(Debug, Clone, es_derive2::Event)]
pub struct PaymentProcessed {
    // Nested field idempotency using dotted path
    #[es(idempotency(id))]
    pub payment: PaymentId,

    // Nested field correlation with explicit status
    #[es(correlation(id, exists))]
    pub user: UserId,

    pub amount: u64,
}
