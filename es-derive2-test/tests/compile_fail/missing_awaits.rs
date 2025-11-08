use es_core::*;

#[derive(Debug, Clone, es_derive2::InjectableEvent)]
pub struct MissingAwaits {
    pub id: String,
}
