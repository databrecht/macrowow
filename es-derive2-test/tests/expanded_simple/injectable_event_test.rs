impl ::es_core::DynEvent for PaymentAuthorizationReleaseRequestDispatched {
    fn name(&self) -> ::es_core::EventName<'static> {
        Self::NAME
    }
}
impl ::es_core::Event for PaymentAuthorizationReleaseRequestDispatched {
    const NAME: ::es_core::EventName<'static> = ::es_core::EventName::new(
        "PaymentAuthorizationReleaseRequestDispatched",
    );
}
impl ::es_core::ExpectsAwaitedSet<TransferRequested>
for PaymentAuthorizationReleaseRequestDispatched {}
impl ::es_core::ExpectsAwaitedSet<PaymentProcessed>
for PaymentAuthorizationReleaseRequestDispatched {}
impl ::es_core::Idempotent for PaymentAuthorizationReleaseRequestDispatched {
    fn get_idempotency_key(
        &self,
    ) -> Result<::es_core::IdempotencyKey, ::es_core::IdempotencyKeyError> {
        let user_parts: Vec<String> = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                self.payment.id.to_string(),
                self.transaction.ref_num.to_string(),
            ]),
        );
        ::es_core::IdempotencyKey::try_new(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "{0}-{1}", "PaymentAuthorizationReleaseRequestDispatched",
                        user_parts.join("-"),
                    ),
                )
            }),
        )
    }
}
impl ::es_core::Correlated for PaymentAuthorizationReleaseRequestDispatched {
    fn get_correlation_id(
        &self,
    ) -> Result<::es_core::CorrelationId, ::es_core::CorrelationIdError> {
        let user_parts: Vec<String> = <[_]>::into_vec(
            ::alloc::boxed::box_new([self.user.id.to_string()]),
        );
        ::es_core::CorrelationId::try_new(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "{0}-{1}", "PaymentAuthorizationReleaseRequestDispatched",
                        user_parts.join("-"),
                    ),
                )
            }),
        )
    }
    fn expected_correlation_group_status(
        &self,
    ) -> ::es_core::ExpectedCorrelationGroupStatus {
        ::es_core::ExpectedCorrelationGroupStatus::Exists
    }
}
impl ::es_core::DynEvent for TransferRequestedResponse {
    fn name(&self) -> ::es_core::EventName<'static> {
        Self::NAME
    }
}
impl ::es_core::Event for TransferRequestedResponse {
    const NAME: ::es_core::EventName<'static> = ::es_core::EventName::new(
        "TransferRequestedResponse",
    );
}
impl ::es_core::DynEvent for PaymentProcessedResponse {
    fn name(&self) -> ::es_core::EventName<'static> {
        Self::NAME
    }
}
impl ::es_core::Event for PaymentProcessedResponse {
    const NAME: ::es_core::EventName<'static> = ::es_core::EventName::new(
        "PaymentProcessedResponse",
    );
}