use deluxe::ExtractAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

#[derive(Debug, ExtractAttributes)]
#[deluxe(attributes(es_register))]
struct RegisterArgs {
    // Currently empty, but ready for future options
}

pub(crate) fn register(_args: TokenStream, input: TokenStream) -> manyhow::Result<TokenStream> {
    let mut input: syn::ItemImpl = syn::parse2(input)?;
    let _args: RegisterArgs = deluxe::extract_attributes(&mut input)?;

    let impl_info = ImplInfo::try_from(&input)?;
    let registration = impl_info.generate_registration();

    Ok(quote!(
        #registration
        #input
    ))
}

struct ImplInfo {
    self_type: syn::Type,
    event_type: syn::Type,
    trait_kind: TraitKind,
}

#[derive(Debug)]
enum TraitKind {
    AggregateApply,
    AggregateHandle,
    SubscriberHandle,
    ProjectorHandle,
}

impl TryFrom<&syn::ItemImpl> for ImplInfo {
    type Error = manyhow::Error;

    fn try_from(input: &syn::ItemImpl) -> Result<Self, Self::Error> {
        let trait_path = input
            .trait_
            .as_ref()
            .ok_or_else(|| {
                manyhow::error_message!(
                    input.impl_token.span(),
                    "es_register requires an impl block with a trait";
                    help = "Expected: impl TraitName<EventType> for SelfType"
                )
            })?
            .1
            .clone();

        let trait_segment = trait_path.segments.last().ok_or_else(|| {
            manyhow::error_message!(
                trait_path.span(),
                "Could not determine trait name"
            )
        })?;

        let trait_name = trait_segment.ident.to_string();
        let trait_kind = match trait_name.as_str() {
            "AggregateApply" => TraitKind::AggregateApply,
            "AggregateHandle" => TraitKind::AggregateHandle,
            "SubscriberHandle" => TraitKind::SubscriberHandle,
            "ProjectorHandle" => TraitKind::ProjectorHandle,
            _ => manyhow::bail!(
                trait_segment.ident.span(),
                "Unknown trait '{}'", trait_name;
                note = "Supported traits: AggregateApply, AggregateHandle, SubscriberHandle, ProjectorHandle"
            ),
        };

        let event_type = extract_trait_type_argument(&trait_segment.arguments)?;

        Ok(ImplInfo {
            self_type: (*input.self_ty).clone(),
            event_type,
            trait_kind,
        })
    }
}

fn extract_trait_type_argument(args: &syn::PathArguments) -> manyhow::Result<syn::Type> {
    match args {
        syn::PathArguments::AngleBracketed(params) => {
            if params.args.len() != 1 {
                manyhow::bail!(
                    params.span(),
                    "Expected exactly 1 generic parameter, found {}", params.args.len()
                );
            }
            match params.args.first() {
                Some(syn::GenericArgument::Type(ty)) => Ok(ty.clone()),
                _ => manyhow::bail!(params.args.span(), "Expected a type argument"),
            }
        }
        syn::PathArguments::None => {
            manyhow::bail!(args.span(), "Missing generic parameter for event type")
        }
        syn::PathArguments::Parenthesized(p) => {
            manyhow::bail!(p.span(), "Expected angle brackets <EventType>, not parentheses")
        }
    }
}

impl ImplInfo {
    fn generate_registration(&self) -> TokenStream {
        let Self {
            self_type,
            event_type,
            trait_kind,
        } = self;

        match trait_kind {
            TraitKind::AggregateApply => quote! {
                ::es_interface::inventory::submit! {
                    ::es_interface::registry::EsInfoEntry::AggregateApply(
                        ::es_interface::registry::AggregateApplyInfo {
                            event_name: <#event_type as ::es_interface::event::Event>::NAME,
                            aggregate_name: <#self_type as ::es_interface::aggregate::NamedAggregate>::NAME,
                            aggregate_apply_fun: ::es_interface::aggregate::dynamic::aggregate_apply_fun::<#self_type, #event_type>,
                            macro_debug_context: ::es_interface::macro_debug_context!(),
                        }
                    )
                }
            },
            TraitKind::AggregateHandle => quote! {
                ::es_interface::inventory::submit! {
                    ::es_interface::registry::EsInfoEntry::AggregateHandle(
                        ::es_interface::registry::AggregateHandleInfo {
                            event_name: <#event_type as ::es_interface::event::Event>::NAME,
                            aggregate_name: <#self_type as ::es_interface::aggregate::NamedAggregate>::NAME,
                            aggregate_handle_fun: ::es_interface::aggregate::dynamic::aggregate_handle_fun::<#self_type, #event_type>,
                            aggregate_output_event_names_fun: <<<#self_type as AggregateHandle<#event_type>>::Output as ::es_interface::output::HandlerOutput>
                                ::Member as ::es::event::EventSet>::possible_names,
                            macro_debug_context: ::es_interface::macro_debug_context!(),
                        }
                    )
                }
            },
            TraitKind::SubscriberHandle => quote! {
                ::es_interface::inventory::submit! {
                    ::es_interface::registry::EsInfoEntry::Subscriber(
                        ::es_interface::registry::SubscriberHandleInfo {
                            event_name: <#event_type as ::es_interface::event::Event>::NAME,
                            subscriber_name: <#self_type as ::es_interface::subscriber::NamedSubscriber>::NAME,
                            subscriber_handle_fun: &(::es_interface::registry::subscriber_handle_fun::<#self_type, #event_type> as ::es_interface::registry::SubscriberHandlerFunction),
                            macro_debug_context: ::es_interface::macro_debug_context!(),
                        }
                    )
                }
            },
            TraitKind::ProjectorHandle => quote! {
                ::es_interface::inventory::submit! {
                    ::es_interface::registry::EsInfoEntry::Projector(
                        ::es_interface::registry::ProjectorHandleInfo {
                            event_name: <#event_type as ::es_interface::event::Event>::NAME,
                            projector_name: <#self_type as ::es_interface::projector::NamedProjector>::NAME,
                            macro_debug_context: ::es_interface::macro_debug_context!(),
                        }
                    )
                }
            },
        }
    }
}
