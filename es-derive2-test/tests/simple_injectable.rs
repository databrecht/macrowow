use es_core::*;

#[derive(Debug, Clone)]
pub struct UserId {
    pub id: String,
}

#[derive(Debug, Clone, es_derive2::InjectableEvent)]
#[es(awaits = [FooEvent])]
#[es(idempotency = ["user.id"])]
pub struct MyEvent {
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
fn test_injectable_event() {
    let event = MyEvent {
        user: UserId {
            id: "123".to_string(),
        },
    };

    // Should compile - has Event trait
    let _name = <MyEvent as Event>::NAME;
}
