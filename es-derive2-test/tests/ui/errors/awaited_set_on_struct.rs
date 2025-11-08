// Error: AwaitedSet can only be used on enums
#[derive(es_derive2::AwaitedSet)]
pub struct NotAnEnum {
    pub data: String,
}

fn main() {}
