use std::collections::BTreeMap;

use deluxe::{ExtractAttributes, ParseMetaItem};
use manyhow::Result;
use quote::quote;
use syn::{DeriveInput, ImplGenerics, TypeGenerics, WhereClause};

pub fn generate_event_impl(
    name: &proc_macro2::Ident,
    impl_generics: &ImplGenerics,
    ty_generics: &TypeGenerics,
    where_clause: Option<&WhereClause>,
) -> proc_macro2::TokenStream {
    quote! {
        #[automatically_derived]
        impl #impl_generics ::es_interface::DynEvent for #name #ty_generics #where_clause {
            fn name(&self) -> ::es_interface::EventName<'static> {
                Self::NAME
            }
        }

        #[automatically_derived]
        impl #impl_generics ::es_interface::Event for #name #ty_generics #where_clause {
            const NAME: ::es_interface::EventName<'static> = ::es_interface::EventName::new(stringify!(#name));
        }

        #[automatically_derived]
        impl #impl_generics ::es_interface::event::Event for #name #ty_generics #where_clause {
            const NAME: ::es_interface::EventName<'static> = ::es_interface::EventName::new(stringify!(#name));
        }
    }
}
