use es_core::*;
pub struct Transferred {
    pub transfer_id: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for Transferred {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "Transferred",
            "transfer_id",
            &&self.transfer_id,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Transferred {
    #[inline]
    fn clone(&self) -> Transferred {
        Transferred {
            transfer_id: ::core::clone::Clone::clone(&self.transfer_id),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Transferred {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Transferred {
    #[inline]
    fn eq(&self, other: &Transferred) -> bool {
        self.transfer_id == other.transfer_id
    }
}
impl ::es_core::DynEvent for Transferred {
    fn name(&self) -> ::es_core::EventName<'static> {
        Self::NAME
    }
}
impl ::es_core::Event for Transferred {
    const NAME: ::es_core::EventName<'static> = ::es_core::EventName::new("Transferred");
}
pub struct TransferFailed {
    pub transfer_id: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for TransferFailed {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "TransferFailed",
            "transfer_id",
            &&self.transfer_id,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TransferFailed {
    #[inline]
    fn clone(&self) -> TransferFailed {
        TransferFailed {
            transfer_id: ::core::clone::Clone::clone(&self.transfer_id),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TransferFailed {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TransferFailed {
    #[inline]
    fn eq(&self, other: &TransferFailed) -> bool {
        self.transfer_id == other.transfer_id
    }
}
impl ::es_core::DynEvent for TransferFailed {
    fn name(&self) -> ::es_core::EventName<'static> {
        Self::NAME
    }
}
impl ::es_core::Event for TransferFailed {
    const NAME: ::es_core::EventName<'static> = ::es_core::EventName::new(
        "TransferFailed",
    );
}
pub enum TransferResponse {
    Transferred(Transferred),
    Failed(TransferFailed),
}
#[automatically_derived]
impl ::core::fmt::Debug for TransferResponse {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            TransferResponse::Transferred(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Transferred",
                    &__self_0,
                )
            }
            TransferResponse::Failed(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Failed", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TransferResponse {
    #[inline]
    fn clone(&self) -> TransferResponse {
        match self {
            TransferResponse::Transferred(__self_0) => {
                TransferResponse::Transferred(::core::clone::Clone::clone(__self_0))
            }
            TransferResponse::Failed(__self_0) => {
                TransferResponse::Failed(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
impl AwaitedSet for TransferResponse {
    const NAME: EventSetName = EventSetName::new("TransferResponse");
    const AWAITABLE_EVENTS: &'static [EventName<'static>] = &[
        <Transferred>::NAME,
        <TransferFailed>::NAME,
    ];
    fn event_name(&self) -> EventName<'static> {
        match self {
            Self::Transferred(e) => e.name(),
            Self::Failed(e) => e.name(),
        }
    }
    fn try_from_envelope(envelope: DynEventEnvelope) -> Result<Self, Report> {
        let event_name = envelope.name;
        #[inline]
        fn downcast_error(event_name: EventName<'static>, type_name: &str) -> Report {
            Report::msg(
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "Failed to downcast event \'{0}\' to type {1}. This indicates a type mismatch in the event system.",
                            event_name.as_str(), type_name,
                        ),
                    )
                }),
            )
        }
        if event_name == <Transferred>::NAME {
            return envelope
                .downcast::<Transferred>()
                .map(Self::Transferred)
                .map_err(|_| downcast_error(event_name, "Transferred"));
        }
        if event_name == <TransferFailed>::NAME {
            return envelope
                .downcast::<TransferFailed>()
                .map(Self::Failed)
                .map_err(|_| downcast_error(event_name, "TransferFailed"));
        }
        Err(
            Report::msg(
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "Event \'{0}\' is not part of {1}. Expected one of: {2:?}",
                            event_name.as_str(), "TransferResponse",
                            Self::AWAITABLE_EVENTS,
                        ),
                    )
                }),
            ),
        )
    }
}
impl HasEventSet<TransferResponse> for Transferred {
    fn into_set(self) -> TransferResponse {
        TransferResponse::Transferred(self)
    }
    fn try_from_set(set: TransferResponse) -> Result<Self, TransferResponse> {
        match set {
            TransferResponse::Transferred(e) => Ok(e),
            TransferResponse::Failed(_) => Err(set),
        }
    }
}
impl HasEventSet<TransferResponse> for TransferFailed {
    fn into_set(self) -> TransferResponse {
        TransferResponse::Failed(self)
    }
    fn try_from_set(set: TransferResponse) -> Result<Self, TransferResponse> {
        match set {
            TransferResponse::Transferred(_) => Err(set),
            TransferResponse::Failed(e) => Ok(e),
        }
    }
}
