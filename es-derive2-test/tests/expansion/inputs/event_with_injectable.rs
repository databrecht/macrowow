use es_interface::*;

#[derive(Debug, Clone, es_derive2::InjectableEvent)]
#[es(awaits = [TransferResponse, PaymentResponse], idempotency = ["request_id"], correlation = ["user_id"])]
pub struct TransferRequested {
    pub request_id: String,
    pub user_id: String,
    pub amount: u64,
}

// Dummy awaited sets for the test
#[derive(Debug, Clone)]
pub enum TransferResponse {}

#[derive(Debug, Clone)]
pub enum PaymentResponse {}
