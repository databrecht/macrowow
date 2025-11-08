// Error: InjectableEvent requires both idempotency and correlation
#[derive(es_derive2::InjectableEvent)]
pub struct MyEvent {
    pub id: String,
}

fn main() {}
