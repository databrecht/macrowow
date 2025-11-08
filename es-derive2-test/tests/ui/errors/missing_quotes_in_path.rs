// Error: Forgot quotes around field path
#[derive(es_derive2::InjectableEvent)]
#[es(idempotency = [user.id], correlation = ["user.id"])]
pub struct MyEvent {
    pub user: User,
}

pub struct User {
    pub id: String,
}

fn main() {}
