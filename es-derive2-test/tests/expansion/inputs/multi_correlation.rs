use es_interface::*;

/// Event with multiple correlation fields to test combination behavior
#[derive(Debug, Clone, es_derive2::InjectableEvent)]
#[es(correlation = ["user_id", "session_id"])]
pub struct MultiCorrelationEvent {
    pub user_id: String,
    pub session_id: String,
    pub data: String,
}
