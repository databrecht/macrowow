use es_interface::*;

#[derive(Debug, Clone)]
pub struct UserId {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct PaymentId {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct TransactionRef {
    pub ref_num: String,
}

// Basic injectable event with all features
#[derive(Debug, Clone, es_derive2::InjectableEvent)]
#[es(awaits = [TransferRequested, PaymentProcessed])]
#[es(idempotency = ["payment.id", "transaction.ref_num"])]
#[es(correlation = ["user.id"], status = { exists })]
pub struct PaymentAuthorizationReleaseRequestDispatched {
    pub payment: PaymentId,
    pub user: UserId,
    pub transaction: TransactionRef,
    pub amount: u64,
}

// Placeholder types for awaits
#[derive(Debug, Clone, es_derive2::InjectableEvent)]
pub struct TransferRequestedResponse {
    pub status: String,
}

#[derive(Debug, Clone, es_derive2::AwaitedSet)]
pub enum TransferRequested {
    Response(TransferRequestedResponse),
}

#[derive(Debug, Clone, es_derive2::InjectableEvent)]
pub struct PaymentProcessedResponse {
    pub status: String,
}

#[derive(Debug, Clone, es_derive2::AwaitedSet)]
pub enum PaymentProcessed {
    Response(PaymentProcessedResponse),
}
