// Error: User types "await" instead of "awaits"
#[derive(es_derive2::InjectableEvent)]
#[es(await = [FooEvent], idempotency = ["id"], correlation = ["id"])]  // ❌ Should be "awaits"
pub struct MyEvent {
    pub id: String,
}

fn main() {}
