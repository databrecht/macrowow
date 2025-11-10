impl ::es_interface::DynEvent for PaymentAuthorizationReleaseRequestDispatched {
    fn name(&self) -> ::es_interface::EventName<'static> {
        Self::NAME
    }
}
impl ::es_interface::Event for PaymentAuthorizationReleaseRequestDispatched {
    const NAME: ::es_interface::EventName<'static> = ::es_interface::EventName::new(
        "PaymentAuthorizationReleaseRequestDispatched",
    );
}
impl ::es_interface::ExpectsAwaitedSet<TransferRequested>
for PaymentAuthorizationReleaseRequestDispatched {}
impl ::es_interface::ExpectsAwaitedSet<PaymentProcessed>
for PaymentAuthorizationReleaseRequestDispatched {}
impl ::es_interface::Idempotent for PaymentAuthorizationReleaseRequestDispatched {
    fn get_idempotency_key(
        &self,
    ) -> Result<::es_interface::IdempotencyKey, ::es_interface::IdempotencyKeyError> {
        let user_parts: Vec<String> = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                self.payment.id.to_string(),
                self.transaction.ref_num.to_string(),
            ]),
        );
        ::es_interface::IdempotencyKey::try_new(
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
impl ::es_interface::Correlated for PaymentAuthorizationReleaseRequestDispatched {
    fn get_correlation_id(
        &self,
    ) -> Result<::es_interface::CorrelationId, ::es_interface::CorrelationIdError> {
        let user_parts: Vec<String> = <[_]>::into_vec(
            ::alloc::boxed::box_new([self.user.id.to_string()]),
        );
        ::es_interface::CorrelationId::try_new(
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
    ) -> ::es_interface::ExpectedCorrelationGroupStatus {
        ::es_interface::ExpectedCorrelationGroupStatus::Exists
    }
}
impl ::es_interface::DynEvent for TransferRequestedResponse {
    fn name(&self) -> ::es_interface::EventName<'static> {
        Self::NAME
    }
}
impl ::es_interface::Event for TransferRequestedResponse {
    const NAME: ::es_interface::EventName<'static> = ::es_interface::EventName::new(
        "TransferRequestedResponse",
    );
}
impl ::es_interface::DynEvent for PaymentProcessedResponse {
    fn name(&self) -> ::es_interface::EventName<'static> {
        Self::NAME
    }
}
impl ::es_interface::Event for PaymentProcessedResponse {
    const NAME: ::es_interface::EventName<'static> = ::es_interface::EventName::new(
        "PaymentProcessedResponse",
    );
}