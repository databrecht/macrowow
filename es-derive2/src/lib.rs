use manyhow::{manyhow, Result};
use proc_macro::TokenStream;
use syn::DeriveInput;

mod awaited_set;
mod event;
mod injectable_event;
mod register;
mod shared;

#[manyhow]
#[proc_macro_derive(AwaitedSet, attributes(es))]
pub fn derive_awaited_set(input: DeriveInput) -> Result {
    awaited_set::awaited_set_impl(input)
}

#[manyhow]
#[proc_macro_derive(Event)]
pub fn derive_event(input: DeriveInput) -> Result {
    event::event_impl(input)
}

#[manyhow]
#[proc_macro_derive(InjectableEvent, attributes(es))]
pub fn derive_injectable_event(input: DeriveInput) -> Result {
    injectable_event::injectable_event_impl(input)
}

#[manyhow]
#[proc_macro_attribute]
pub fn es_register(args: TokenStream, input: TokenStream) -> manyhow::Result<TokenStream> {
    register::register(args.into(), input.into()).map(Into::into)
}
