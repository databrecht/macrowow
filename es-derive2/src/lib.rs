use manyhow::{manyhow, Result};
use syn::DeriveInput;

mod awaited_set;
mod event;
mod injectable_event;

#[manyhow]
#[proc_macro_derive(AwaitedSet, attributes(es))]
pub fn derive_awaited_set(input: DeriveInput) -> Result {
    awaited_set::awaited_set_impl(input)
}

#[manyhow]
#[proc_macro_derive(Event, attributes(es))]
pub fn derive_event(input: DeriveInput) -> Result {
    event::event_impl(input)
}

#[manyhow]
#[proc_macro_derive(InjectableEvent, attributes(es))]
pub fn derive_injectable_event(input: DeriveInput) -> Result {
    injectable_event::injectable_event_impl(input)
}
