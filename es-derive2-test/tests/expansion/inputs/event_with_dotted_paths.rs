use es_interface::*;

#[derive(Debug, Clone)]
pub struct UserId {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct PaymentId {
    pub id: String,
}

/// Event demonstrating dotted path access for idempotency and correlation
#[derive(Debug, Clone, es_derive2::InjectableEvent)]
#[es(idempotency = ["payment.id"], correlation = ["user.id"], status = { exists })]
pub struct PaymentProcessed {
    pub payment: PaymentId,
    pub user: UserId,
    pub amount: u64,
}
