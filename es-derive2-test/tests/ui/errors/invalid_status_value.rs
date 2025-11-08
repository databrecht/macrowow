// Error: Invalid status value (should be "new", "exists", or "any")
#[derive(es_derive2::InjectableEvent)]
#[es(idempotency = ["user.id"], correlation = ["user.id"], status = { invalid })]
pub struct MyEvent {
    pub user: User,
}

pub struct User {
    pub id: String,
}

fn main() {}
