// Error: awaits expects array syntax
#[derive(es_derive2::InjectableEvent)]
#[es(awaits = FooEvent, idempotency = ["id"], correlation = ["id"])]
pub struct MyEvent {
    pub id: String,
}

fn main() {}
