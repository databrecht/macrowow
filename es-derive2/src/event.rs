use manyhow::Result;
use syn::DeriveInput;

use crate::shared::generate_event_impl;

const USAGE_EXAMPLE: &str = r#"

Example usage:
  #[derive(Event)]
  pub struct PaymentRequested { ... }

"#;

pub(crate) fn event_impl(input: DeriveInput) -> Result {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let event_impl = generate_event_impl(name, &impl_generics, &ty_generics, where_clause);
    Ok(event_impl)
}
