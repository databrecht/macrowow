// Error: Variants must use newtype pattern (single unnamed field)
#[derive(es_derive2::AwaitedSet)]
pub enum BadResponse {
    Success { data: String },  // Named fields not allowed
}

fn main() {}
