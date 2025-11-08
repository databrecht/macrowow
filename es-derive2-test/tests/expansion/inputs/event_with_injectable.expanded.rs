use es_core::*;
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
impl ::es_core::DynEvent for TransferRequested {
    fn name(&self) -> ::es_core::EventName<'static> {
        Self::NAME
    }
}
#[automatically_derived]
impl ::es_core::Event for TransferRequested {
    const NAME: ::es_core::EventName<'static> = ::es_core::EventName::new(
        "TransferRequested",
    );
}
#[automatically_derived]
impl ::es_core::Idempotent for TransferRequested
where
    String: std::fmt::Display,
{
    fn get_idempotency_key(
        &self,
    ) -> Result<::es_core::IdempotencyKey, ::es_core::IdempotencyKeyError> {
        let user_parts: Vec<String> = <[_]>::into_vec(
            ::alloc::boxed::box_new([self.request_id.to_string()]),
        );
        ::es_core::IdempotencyKey::try_new(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}-{1}", "TransferRequested", user_parts.join("-")),
                )
            }),
        )
    }
}
#[automatically_derived]
impl ::es_core::Correlated for TransferRequested
where
    String: std::fmt::Display,
{
    fn get_correlation_id(
        &self,
    ) -> Result<::es_core::CorrelationId, ::es_core::CorrelationIdError> {
        let user_parts: Vec<String> = <[_]>::into_vec(
            ::alloc::boxed::box_new([self.user_id.to_string()]),
        );
        ::es_core::CorrelationId::try_new(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}-{1}", "TransferRequested", user_parts.join("-")),
                )
            }),
        )
    }
    fn expected_correlation_group_status(
        &self,
    ) -> ::es_core::ExpectedCorrelationGroupStatus {
        ::es_core::ExpectedCorrelationGroupStatus::New
    }
}
#[automatically_derived]
impl ::es_core::AwaitableFor<TransferResponse> for TransferRequested {}
impl ::es_core::AwaitableFor<PaymentResponse> for TransferRequested {}
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
