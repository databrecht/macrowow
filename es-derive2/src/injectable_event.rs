use deluxe::{ExtractAttributes, ParseMetaItem};
use manyhow::Result;
use quote::quote;
use syn::DeriveInput;
use std::collections::BTreeMap;

/// Parse a dotted path string like "user.id" into a token stream for field access.
///
/// Converts "user.id" -> self.user.id
fn parse_dotted_path(path_str: &str) -> syn::Result<proc_macro2::TokenStream> {
    let segments: Vec<_> = path_str.split('.').collect();

    if segments.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Path cannot be empty"
        ));
    }

    let idents: Vec<syn::Ident> = segments
        .iter()
        .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()))
        .collect();

    Ok(quote!(self.#(#idents).*))
}

/// Correlation status determining expected state of correlation group.
#[derive(ParseMetaItem, Debug, Clone, Copy, PartialEq, Eq, Default)]
enum CorrelationStatus {
    /// Correlation group should be new (first event in the group)
    #[default]
    #[deluxe(rename = new)]
    New,
    /// Correlation group should already exist
    #[deluxe(rename = exists)]
    Exists,
    /// Correlation group can be new or existing
    #[deluxe(rename = any)]
    Any,
}

#[derive(ExtractAttributes)]
#[deluxe(attributes(es))]
struct InjectableEventAttrs {
    /// Event sets this event awaits a response from
    ///
    /// Example: #[es(awaits = [TransferResponse, PaymentResponse])]
    #[deluxe(default)]
    awaits: Option<Vec<syn::Path>>,

    /// Field paths for generating idempotency keys (required)
    ///
    /// Example: #[es(idempotency = ["user.id", "payment.id"])]
    idempotency: Vec<syn::LitStr>,

    /// Field paths for generating correlation IDs (required)
    ///
    /// Example: #[es(correlation = ["user.id"])]
    correlation: Vec<syn::LitStr>,

    /// Expected correlation group status: new, exists, or any
    ///
    /// Example: #[es(status = { exists })]
    #[deluxe(default)]
    status: Option<CorrelationStatus>,
}

const USAGE_EXAMPLE: &str = r#"

Example usage:
  #[derive(InjectableEvent)]
  #[es(awaits = [PaymentResponse], idempotency = ["user.id"], correlation = ["transaction_id"], status = { exists })]
  pub struct PaymentRequested { ... }

"#;

pub(crate) fn injectable_event_impl(mut input: DeriveInput) -> Result {
    // Extract attributes using Deluxe
    let attrs: InjectableEventAttrs = deluxe::extract_attributes(&mut input).map_err(|e| {
        syn::Error::new(e.span(), format!("{}{}", e, USAGE_EXAMPLE))
    })?;

    let name = &input.ident;

    // Validate that arrays are not empty
    if attrs.idempotency.is_empty() {
        return Err(syn::Error::new_spanned(
            &input,
            format!("idempotency array cannot be empty{}", USAGE_EXAMPLE)
        ).into());
    }
    if attrs.correlation.is_empty() {
        return Err(syn::Error::new_spanned(
            &input,
            format!("correlation array cannot be empty{}", USAGE_EXAMPLE)
        ).into());
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Basic Event trait implementations
    let mut event_impl = quote! {
        #[automatically_derived]
        impl #impl_generics ::es_core::DynEvent for #name #ty_generics #where_clause {
            fn name(&self) -> ::es_core::EventName<'static> {
                Self::NAME
            }
        }

        #[automatically_derived]
        impl #impl_generics ::es_core::Event for #name #ty_generics #where_clause {
            const NAME: ::es_core::EventName<'static> = ::es_core::EventName::new(stringify!(#name));
        }
    };

    // Add ExpectsAwaitedSet impls if awaits attribute is present
    if let Some(awaited_sets) = &attrs.awaits {
        let awaited_sets_vec: Vec<_> = awaited_sets.iter().collect();
        event_impl.extend(quote! {
            #[automatically_derived]
            #(
                impl #impl_generics ::es_core::ExpectsAwaitedSet<#awaited_sets_vec> for #name #ty_generics #where_clause {}
            )*
        });
    }

    let mut output = event_impl;

    // Implement Idempotent (required)
    let idempotency_impl = generate_idempotency_impl(&name, &impl_generics, &ty_generics, &where_clause, &attrs.idempotency)?;
    output.extend(idempotency_impl);

    // Implement Correlated (required)
    let correlation_impl = generate_correlation_impl(&name, &impl_generics, &ty_generics, &where_clause, &attrs.correlation, &attrs.status)?;
    output.extend(correlation_impl);

    Ok(output)
}

/// Generate the `Idempotent` trait implementation.
///
/// Takes struct-level paths from `#[es(idempotency = ["payment.id", "transaction.ref_num"])]`
/// and generates a key by concatenating their string representations.
fn generate_idempotency_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
    idempotency_paths: &Vec<syn::LitStr>,
) -> Result<proc_macro2::TokenStream> {
    let mut field_accessors = BTreeMap::new();

    for path_str in idempotency_paths {
        let field_accessor = parse_dotted_path(&path_str.value())?;
        field_accessors.insert(field_accessor.to_string(), field_accessor);
    }

    let field_accessor_iter = field_accessors.values();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::es_core::Idempotent for #name #ty_generics #where_clause {
            fn get_idempotency_key(&self) -> Result<::es_core::IdempotencyKey, ::es_core::IdempotencyKeyError> {
                let user_parts: Vec<String> = vec![#(#field_accessor_iter.to_string()),*];
                ::es_core::IdempotencyKey::try_new(format!("{}-{}", stringify!(#name), user_parts.join("-")))
            }
        }
    })
}

/// Generate the `Correlated` trait implementation.
///
/// Takes struct-level paths from `#[es(correlation = ["user.id", "session.id"], status = { exists })]`
/// and generates a correlation ID by concatenating their string representations.
fn generate_correlation_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
    correlation_paths: &Vec<syn::LitStr>,
    status: &Option<CorrelationStatus>,
) -> Result<proc_macro2::TokenStream> {
    let mut field_accessors = BTreeMap::new();

    for path_str in correlation_paths {
        let field_accessor = parse_dotted_path(&path_str.value())?;
        field_accessors.insert(field_accessor.to_string(), field_accessor);
    }

    let field_accessor_iter = field_accessors.values();

    // status is Option<CorrelationStatus>, default to New
    let status_token = match status.unwrap_or(CorrelationStatus::New) {
        CorrelationStatus::New => quote!(::es_core::ExpectedCorrelationGroupStatus::New),
        CorrelationStatus::Exists => quote!(::es_core::ExpectedCorrelationGroupStatus::Exists),
        CorrelationStatus::Any => quote!(::es_core::ExpectedCorrelationGroupStatus::Any),
    };

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::es_core::Correlated for #name #ty_generics #where_clause {
            fn get_correlation_id(&self) -> Result<::es_core::CorrelationId, ::es_core::CorrelationIdError> {
                let user_parts: Vec<String> = vec![#(#field_accessor_iter.to_string()),*];
                ::es_core::CorrelationId::try_new(format!("{}-{}", stringify!(#name), user_parts.join("-")))
            }

            fn expected_correlation_group_status(&self) -> ::es_core::ExpectedCorrelationGroupStatus {
                #status_token
            }
        }
    })
}
