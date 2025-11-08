use manyhow::{ensure, Result};
use quote::quote;
use syn::DeriveInput;

const USAGE_EXAMPLE: &str = r#"
Usage example:
#[derive(AwaitedSet)]
enum TransferResponse {
    Transferred(Transferred),
    Failed(TransferFailed),
}

"#;

pub(crate) fn awaited_set_impl(input: DeriveInput) -> Result {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let attrs = &input.attrs;

    // Extract variants from enum
    let data_enum = match &input.data {
        syn::Data::Enum(data) => data,
        _ => {
            return Err(syn::Error::new_spanned(
                &name,
                format!("AwaitedSet only supports enums. {}", USAGE_EXAMPLE)
            ).into());
        }
    };

    let mut variant_info = Vec::new();
    for variant in &data_enum.variants {
        let variant_name = &variant.ident;

        // Ensure variant has exactly one unnamed field (newtype pattern)
        let event_type = match &variant.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                &fields.unnamed.first().unwrap().ty
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    variant_name,
                    format!("Each enum variant must wrap exactly one event type. {}", USAGE_EXAMPLE)
                ).into());
            }
        };

        variant_info.push((variant_name, event_type));
    }

    // Validate no duplicate event types
    {
        let mut seen = std::collections::HashSet::new();
        for (_variant_name, event_type) in &variant_info {
            let type_str = quote!(#event_type).to_string();
            ensure!(
                seen.insert(type_str.clone()),
                event_type,
                "Duplicate event type '{}' in AwaitedSet. Each event type can only appear once.",
                type_str
            );
        }
    }

    // Generate match arms for AwaitedSet trait
    let name_match_arms = variant_info.iter().map(|(variant, _)| {
        quote! { Self::#variant(e) => e.name() }
    });

    let awaitable_events_items: Vec<_> = variant_info.iter().map(|(_, event_type)| {
        quote! { <#event_type>::NAME }
    }).collect();

    let try_from_envelope_arms = variant_info.iter().map(|(variant, event_type)| {
        quote! {
            if event_name == <#event_type>::NAME {
                return envelope.downcast::<#event_type>()
                    .map(Self::#variant)
                    .map_err(|_| downcast_error(event_name, stringify!(#event_type)));
            }
        }
    });

    // Generate HasEventSet impls for each event type
    let has_event_set_impls = variant_info.iter().map(|(variant, event_type)| {
        let try_from_set_arms = variant_info.iter().map(|(v, _)| {
            if v == variant {
                quote! { #name::#v(e) => Ok(e) }
            } else {
                quote! { #name::#v(_) => Err(set) }
            }
        });

        quote! {
            impl #impl_generics HasEventSet<#name #ty_generics> for #event_type #where_clause {
                fn into_set(self) -> #name #ty_generics {
                    #name::#variant(self)
                }

                fn try_from_set(set: #name #ty_generics) -> Result<Self, #name #ty_generics> {
                    match set {
                        #(#try_from_set_arms,)*
                    }
                }
            }
        }
    });

    let expanded = quote! {
        #(#attrs)*
        impl #impl_generics AwaitedSet for #name #ty_generics #where_clause {
            const NAME: EventSetName = EventSetName::new(stringify!(#name));
            const AWAITABLE_EVENTS: &'static [EventName<'static>] = &[#(#awaitable_events_items,)*];

            fn event_name(&self) -> EventName<'static> {
                match self {
                    #(#name_match_arms,)*
                }
            }

            fn try_from_envelope(envelope: DynEventEnvelope) -> Result<Self, Report> {
                let event_name = envelope.name;

                #[inline]
                fn downcast_error(event_name: EventName<'static>, type_name: &str) -> Report {
                    Report::msg(format!(
                        "Failed to downcast event '{}' to type {}. This indicates a type mismatch in the event system.",
                        event_name.as_str(),
                        type_name
                    ))
                }

                #(#try_from_envelope_arms)*
                Err(Report::msg(format!(
                    "Event '{}' is not part of {}. Expected one of: {:?}",
                    event_name.as_str(),
                    stringify!(#name),
                    Self::AWAITABLE_EVENTS
                )))
            }
        }

        #(#has_event_set_impls)*
    };

    Ok(expanded)
}
