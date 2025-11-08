use darling::{ast, FromDeriveInput, FromField, FromMeta};
use darling::ast::NestedMeta;
use darling::error::Accumulator;
use manyhow::Result;
use quote::quote;
use syn::DeriveInput;
use std::collections::BTreeMap;

/// Correlation status determining expected state of correlation group.
#[derive(FromMeta, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[darling(rename_all = "lowercase")]
enum CorrelationStatus {
    /// Correlation group should be new (first event in the group)
    #[default]
    New,
    /// Correlation group should already exist
    Exists,
    /// Correlation group can be new or existing
    Any,
}

/// Idempotency field configuration.
///
/// Supports either using the field directly or accessing a nested field via dotted path.
#[derive(Debug, Clone, PartialEq, Eq)]
enum IdempotencyPropParams {
    /// Use the field directly: `#[es(idempotency)]`
    Default,
    /// Access a nested field: `#[es(idempotency(user.id))]`
    Path(syn::Path),
}

impl FromMeta for IdempotencyPropParams {
    fn from_word() -> darling::Result<Self> {
        Ok(IdempotencyPropParams::Default)
    }

    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        if items.len() == 1 {
            if let NestedMeta::Meta(syn::Meta::Path(path)) = &items[0] {
                return Ok(IdempotencyPropParams::Path(path.clone()));
            }
        }
        Err(darling::Error::custom(
            "idempotency expects either no arguments or a single path like `idempotency(user.id)`"
        ))
    }
}

/// Correlation field configuration.
///
/// Supports using the field directly or accessing a nested field via dotted path,
/// optionally with a correlation status.
#[derive(Debug, Clone, PartialEq, Eq)]
enum CorrelationPropParams {
    /// Use the field directly: `#[es(correlation)]`
    Default,
    /// Access nested field with optional status: `#[es(correlation(user.id))]` or `#[es(correlation(user.id, exists))]`
    Path { path: syn::Path, status: CorrelationStatus },
}

impl FromMeta for CorrelationPropParams {
    fn from_word() -> darling::Result<Self> {
        Ok(CorrelationPropParams::Default)
    }

    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        match items.len() {
            1 => {
                // #[es(correlation(user.id))]
                if let NestedMeta::Meta(syn::Meta::Path(path)) = &items[0] {
                    return Ok(CorrelationPropParams::Path {
                        path: path.clone(),
                        status: CorrelationStatus::New,
                    });
                }
                Err(darling::Error::custom(
                    "correlation expects a path like `correlation(user.id)`"
                ))
            }
            2 => {
                // #[es(correlation(user.id, exists))]
                if let NestedMeta::Meta(syn::Meta::Path(path)) = &items[0] {
                    if let NestedMeta::Meta(syn::Meta::Path(status_path)) = &items[1] {
                        let status_str = status_path.get_ident()
                            .ok_or_else(|| darling::Error::custom("status must be a single identifier"))?
                            .to_string();
                        let status = match status_str.as_str() {
                            "new" => CorrelationStatus::New,
                            "exists" => CorrelationStatus::Exists,
                            "any" => CorrelationStatus::Any,
                            _ => return Err(darling::Error::custom(
                                format!("unknown correlation status '{}', expected: new, exists, or any", status_str)
                            )),
                        };
                        return Ok(CorrelationPropParams::Path {
                            path: path.clone(),
                            status,
                        });
                    }
                }
                Err(darling::Error::custom(
                    "correlation expects path and status like `correlation(user.id, exists)`"
                ))
            }
            _ => Err(darling::Error::custom(
                "correlation expects either no arguments, a single path, or path with status"
            ))
        }
    }
}

#[derive(FromField, Clone)]
#[darling(attributes(es), forward_attrs(allow, cfg, clippy))]
struct EventFieldParams {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    attrs: Vec<syn::Attribute>,
    idempotency: Option<IdempotencyPropParams>,
    correlation: Option<CorrelationPropParams>,
}

#[derive(FromDeriveInput, Clone)]
#[darling(
    attributes(es),
    forward_attrs(allow, cfg, clippy),
    supports(struct_any)
)]
struct EventDerive {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    data: ast::Data<(), EventFieldParams>,
    generics: syn::Generics,
    injectable: Option<InjectableParams>,
}

#[derive(Debug, Clone, darling::FromMeta)]
struct InjectableParams {
    awaits: darling::util::PathList,
}

impl EventDerive {
    fn try_new(input: DeriveInput) -> Result<Self> {
        Ok(Self::from_derive_input(&input).map_err(|e| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("{}", e)
            )
        })?)
    }
}

pub(crate) fn event_impl(input: DeriveInput) -> Result {
    let derived = EventDerive::try_new(input)?;

    let name = &derived.ident;
    let (impl_generics, ty_generics, where_clause) = derived.generics.split_for_impl();
    let attrs = &derived.attrs;

    // Basic Event trait implementation
    let event_impl = quote! {
        #(#attrs)*
        #[automatically_derived]
        impl #impl_generics ::es_core::DynEvent for #name #ty_generics #where_clause {
            fn name(&self) -> ::es_core::EventName<'static> {
                Self::NAME
            }
        }

        #(#attrs)*
        #[automatically_derived]
        impl #impl_generics ::es_core::Event for #name #ty_generics #where_clause {
            const NAME: ::es_core::EventName<'static> = ::es_core::EventName::new(stringify!(#name));
        }
    };

    let mut output = event_impl;

    // Get fields
    let fields = derived.data.as_ref().take_struct()
        .ok_or_else(|| syn::Error::new_spanned(
            &name,
            "Event derive only supports structs"
        ))?;

    // Implement Idempotent if any field has #[es(idempotency)]
    if let Some(idempotency_impl) = generate_idempotency_impl(&derived, &fields)? {
        output.extend(idempotency_impl);
    }

    // Implement Correlated if any field has #[es(correlation)]
    if let Some(correlation_impl) = generate_correlation_impl(&derived, &fields)? {
        output.extend(correlation_impl);
    }

    // Implement Injectable if #[es(injectable(awaits(...)))] is present
    if let Some(injectable) = &derived.injectable {
        let awaited_sets = &injectable.awaits;

        output.extend(quote! {
            #(#attrs)*
            #[automatically_derived]
            #(
                impl #impl_generics ::es_core::AwaitableFor<#awaited_sets> for #name #ty_generics #where_clause {}
            )*
        });
    }

    Ok(output)
}

/// Generate the `Idempotent` trait implementation for a struct.
///
/// Collects all fields marked with `#[es(idempotency)]` or `#[es(idempotency(path))]`
/// and generates a key by concatenating their string representations.
fn generate_idempotency_impl(
    derived: &EventDerive,
    fields: &ast::Fields<&EventFieldParams>,
) -> Result<Option<proc_macro2::TokenStream>> {
    let name = &derived.ident;
    let (impl_generics, ty_generics, where_clause) = derived.generics.split_for_impl();
    let attrs = &derived.attrs;

    let mut field_accessors = BTreeMap::new();
    let mut field_types = Vec::new();

    for (idx, field) in fields.iter().enumerate() {
        if let Some(idempotency_params) = &field.idempotency {
            let base_field_ident = field.ident.as_ref()
                .map(|i| quote!(#i))
                .unwrap_or_else(|| {
                    let idx = syn::Index::from(idx);
                    quote!(#idx)
                });

            let field_accessor = match idempotency_params {
                IdempotencyPropParams::Default => {
                    // Direct field access: self.field_name
                    quote!(self.#base_field_ident)
                }
                IdempotencyPropParams::Path(path) => {
                    // Dotted path access: self.field_name.nested.field
                    quote!(self.#base_field_ident.#path)
                }
            };

            field_accessors.insert(field_accessor.to_string(), field_accessor);
            field_types.push(&field.ty);
        }
    }

    if field_accessors.is_empty() {
        return Ok(None);
    }

    let field_accessor_iter = field_accessors.values();

    Ok(Some(quote! {
        #(#attrs)*
        #[automatically_derived]
        impl #impl_generics ::es_core::Idempotent for #name #ty_generics
        where
            #(#field_types: std::fmt::Display,)*
            #where_clause
        {
            fn get_idempotency_key(&self) -> Result<::es_core::IdempotencyKey, ::es_core::IdempotencyKeyError> {
                let user_parts: Vec<String> = vec![#(#field_accessor_iter.to_string()),*];
                ::es_core::IdempotencyKey::try_new(format!("{}-{}", stringify!(#name), user_parts.join("-")))
            }
        }
    }))
}

/// Generate the `Correlated` trait implementation for a struct.
///
/// Collects all fields marked with `#[es(correlation)]` or `#[es(correlation(path, status))]`
/// and generates a correlation ID by concatenating their string representations.
/// The correlation status defaults to `New` but can be overridden per-field.
fn generate_correlation_impl(
    derived: &EventDerive,
    fields: &ast::Fields<&EventFieldParams>,
) -> Result<Option<proc_macro2::TokenStream>> {
    let name = &derived.ident;
    let (impl_generics, ty_generics, where_clause) = derived.generics.split_for_impl();
    let attrs = &derived.attrs;

    let mut field_accessors = BTreeMap::new();
    let mut field_types = Vec::new();
    let mut correlation_status = CorrelationStatus::New; // Default status

    for (idx, field) in fields.iter().enumerate() {
        if let Some(correlation_params) = &field.correlation {
            let base_field_ident = field.ident.as_ref()
                .map(|i| quote!(#i))
                .unwrap_or_else(|| {
                    let idx = syn::Index::from(idx);
                    quote!(#idx)
                });

            let (field_accessor, status) = match correlation_params {
                CorrelationPropParams::Default => {
                    // Direct field access: self.field_name
                    (quote!(self.#base_field_ident), CorrelationStatus::New)
                }
                CorrelationPropParams::Path { path, status } => {
                    // Dotted path access: self.field_name.nested.field
                    let accessor = quote!(self.#base_field_ident.#path);
                    (accessor, *status)
                }
            };

            field_accessors.insert(field_accessor.to_string(), field_accessor);
            field_types.push(&field.ty);

            // Use the first field's status (or could accumulate and check for conflicts)
            if field_accessors.len() == 1 {
                correlation_status = status;
            }
        }
    }

    if field_accessors.is_empty() {
        return Ok(None);
    }

    let field_accessor_iter = field_accessors.values();
    let status_token = match correlation_status {
        CorrelationStatus::New => quote!(::es_core::ExpectedCorrelationGroupStatus::New),
        CorrelationStatus::Exists => quote!(::es_core::ExpectedCorrelationGroupStatus::Exists),
        CorrelationStatus::Any => quote!(::es_core::ExpectedCorrelationGroupStatus::Any),
    };

    Ok(Some(quote! {
        #(#attrs)*
        #[automatically_derived]
        impl #impl_generics ::es_core::Correlated for #name #ty_generics
        where
            #(#field_types: std::fmt::Display,)*
            #where_clause
        {
            fn get_correlation_id(&self) -> Result<::es_core::CorrelationId, ::es_core::CorrelationIdError> {
                let user_parts: Vec<String> = vec![#(#field_accessor_iter.to_string()),*];
                ::es_core::CorrelationId::try_new(format!("{}-{}", stringify!(#name), user_parts.join("-")))
            }

            fn expected_correlation_group_status(&self) -> ::es_core::ExpectedCorrelationGroupStatus {
                #status_token
            }
        }
    }))
}
