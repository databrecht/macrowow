use es_core::*;

#[derive(Debug, Clone, es_derive2::Event)]
#[es(injectable(awaits(TransferResponse, PaymentResponse)))]
pub struct TransferRequested {
    #[es(idempotency)]
    pub request_id: String,
    #[es(correlation)]
    pub user_id: String,
    pub amount: u64,
}

// Dummy awaited sets for the test
#[derive(Debug, Clone)]
pub enum TransferResponse {}

#[derive(Debug, Clone)]
pub enum PaymentResponse {}
