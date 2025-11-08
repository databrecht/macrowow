// Error: Empty idempotency and correlation arrays should fail
#[derive(es_derive2::InjectableEvent)]
#[es(idempotency = [], correlation = [])]
pub struct MyEvent {
    pub id: String,
}

fn main() {}
