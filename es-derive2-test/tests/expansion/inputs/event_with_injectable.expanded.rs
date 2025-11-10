use es_interface::*;
#[es(injectable(awaits(TransferResponse, PaymentResponse)))]
pub struct TransferRequested {
    #[es(idempotency)]
    pub request_id: String,
    #[es(correlation)]
    pub user_id: String,
    pub amount: u64,
}
#[automatically_derived]
impl ::core::fmt::Debug for TransferRequested {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "TransferRequested",
            "request_id",
            &self.request_id,
            "user_id",
            &self.user_id,
            "amount",
            &&self.amount,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TransferRequested {
    #[inline]
    fn clone(&self) -> TransferRequested {
        TransferRequested {
            request_id: ::core::clone::Clone::clone(&self.request_id),
            user_id: ::core::clone::Clone::clone(&self.user_id),
            amount: ::core::clone::Clone::clone(&self.amount),
        }
    }
}
#[automatically_derived]
impl ::es_interface::DynEvent for TransferRequested {
    fn name(&self) -> ::es_interface::EventName<'static> {
        Self::NAME
    }
}
#[automatically_derived]
impl ::es_interface::Event for TransferRequested {
    const NAME: ::es_interface::EventName<'static> = ::es_interface::EventName::new(
        "TransferRequested",
    );
}
#[automatically_derived]
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
#[automatically_derived]
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
#[automatically_derived]
impl ::es_interface::AwaitableFor<TransferResponse> for TransferRequested {}
impl ::es_interface::AwaitableFor<PaymentResponse> for TransferRequested {}
pub enum TransferResponse {}
#[automatically_derived]
impl ::core::fmt::Debug for TransferResponse {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TransferResponse {
    #[inline]
    fn clone(&self) -> TransferResponse {
        match *self {}
    }
}
pub enum PaymentResponse {}
#[automatically_derived]
impl ::core::fmt::Debug for PaymentResponse {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {}
    }
}
#[automatically_derived]
impl ::core::clone::Clone for PaymentResponse {
    #[inline]
    fn clone(&self) -> PaymentResponse {
        match *self {}
    }
}
