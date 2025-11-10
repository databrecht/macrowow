impl ::es_interface::DynEvent for TransferRequested {
    fn name(&self) -> ::es_interface::EventName<'static> {
        Self::NAME
    }
}
impl ::es_interface::Event for TransferRequested {
    const NAME: ::es_interface::EventName<'static> = ::es_interface::EventName::new(
        "TransferRequested",
    );
}
impl ::es_interface::Idempotent for TransferRequested
where
    String: std::fmt::Display,
{
    fn get_idempotency_key(
        &self,
    ) -> Result<::es_interface::IdempotencyKey, ::es_interface::IdempotencyKeyError> {
        let user_parts: Vec<String> = <[_]>::into_vec(
            ::alloc::boxed::box_new([self.request_id.to_string()]),
        );
        ::es_interface::IdempotencyKey::try_new(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}-{1}", "TransferRequested", user_parts.join("-")),
                )
            }),
        )
    }
}
impl ::es_interface::Correlated for TransferRequested
where
    String: std::fmt::Display,
{
    fn get_correlation_id(
        &self,
    ) -> Result<::es_interface::CorrelationId, ::es_interface::CorrelationIdError> {
        let user_parts: Vec<String> = <[_]>::into_vec(
            ::alloc::boxed::box_new([self.user_id.to_string()]),
        );
        ::es_interface::CorrelationId::try_new(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}-{1}", "TransferRequested", user_parts.join("-")),
                )
            }),
        )
    }
    fn expected_correlation_group_status(
        &self,
    ) -> ::es_interface::ExpectedCorrelationGroupStatus {
        ::es_interface::ExpectedCorrelationGroupStatus::New
    }
}
impl ::es_interface::AwaitableFor<TransferResponse> for TransferRequested {}
impl ::es_interface::AwaitableFor<PaymentResponse> for TransferRequested {}