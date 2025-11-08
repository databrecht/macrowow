use es_core::*;

/// Event with multiple correlation fields to test combination behavior
#[derive(Debug, Clone, es_derive2::Event)]
pub struct MultiCorrelationEvent {
    // First correlation field with status "new"
    #[es(correlation)]
    pub user_id: String,

    // Second correlation field with status "exists"
    #[es(correlation)]
    pub session_id: String,

    pub data: String,
}
