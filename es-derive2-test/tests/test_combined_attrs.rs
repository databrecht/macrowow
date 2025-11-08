use es_core::*;

#[derive(Debug, Clone)]
pub struct UserId {
    pub id: String,
}

// Test: All attributes in ONE line
#[derive(Debug, Clone, es_derive2::InjectableEvent)]
#[es(awaits = [FooEvent], idempotency = ["user.id"], correlation = ["user.id"], status = { exists })]
pub struct CombinedAttrs {
    pub user: UserId,
}

// Test: All attributes SEPARATED
#[derive(Debug, Clone, es_derive2::InjectableEvent)]
#[es(awaits = [FooEvent])]
#[es(idempotency = ["user.id"])]
#[es(correlation = ["user.id"])]
#[es(status = { exists })]
pub struct SeparatedAttrs {
    pub user: UserId,
}

// Test: Mixed (some combined, some separated)
#[derive(Debug, Clone, es_derive2::InjectableEvent)]
#[es(awaits = [FooEvent], idempotency = ["user.id"])]
#[es(correlation = ["user.id"], status = { exists })]
pub struct MixedAttrs {
    pub user: UserId,
}

#[derive(Debug, Clone, es_derive2::Event)]
pub struct FooResponse {
    pub data: String,
}

#[derive(Debug, Clone, es_derive2::AwaitedSet)]
pub enum FooEvent {
    Response(FooResponse),
}

#[test]
fn test_combined() {
    let event = CombinedAttrs {
        user: UserId { id: "123".to_string() },
    };
    let _name = <CombinedAttrs as Event>::NAME;
    let _key = event.get_idempotency_key().unwrap();
    let _corr = event.get_correlation_id().unwrap();
    let _status = event.expected_correlation_group_status();
}

#[test]
fn test_separated() {
    let event = SeparatedAttrs {
        user: UserId { id: "123".to_string() },
    };
    let _name = <SeparatedAttrs as Event>::NAME;
    let _key = event.get_idempotency_key().unwrap();
    let _corr = event.get_correlation_id().unwrap();
    let _status = event.expected_correlation_group_status();
}

#[test]
fn test_mixed() {
    let event = MixedAttrs {
        user: UserId { id: "123".to_string() },
    };
    let _name = <MixedAttrs as Event>::NAME;
    let _key = event.get_idempotency_key().unwrap();
    let _corr = event.get_correlation_id().unwrap();
    let _status = event.expected_correlation_group_status();
}
