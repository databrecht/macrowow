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