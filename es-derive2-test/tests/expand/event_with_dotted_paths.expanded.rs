use es_core::*;
pub struct UserId {
    pub id: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for UserId {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "UserId", "id", &&self.id)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for UserId {
    #[inline]
    fn clone(&self) -> UserId {
        UserId {
            id: ::core::clone::Clone::clone(&self.id),
        }
    }
}
pub struct PaymentId {
    pub id: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for PaymentId {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "PaymentId",
            "id",
            &&self.id,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for PaymentId {
    #[inline]
    fn clone(&self) -> PaymentId {
        PaymentId {
            id: ::core::clone::Clone::clone(&self.id),
        }
    }
}
/// Event demonstrating dotted path access for idempotency and correlation
pub struct PaymentProcessed {
    #[es(idempotency(id))]
    pub payment: PaymentId,
    #[es(correlation(id, exists))]
    pub user: UserId,
    pub amount: u64,
}
#[automatically_derived]
impl ::core::fmt::Debug for PaymentProcessed {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "PaymentProcessed",
            "payment",
            &self.payment,
            "user",
            &self.user,
            "amount",
            &&self.amount,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for PaymentProcessed {
    #[inline]
    fn clone(&self) -> PaymentProcessed {
        PaymentProcessed {
            payment: ::core::clone::Clone::clone(&self.payment),
            user: ::core::clone::Clone::clone(&self.user),
            amount: ::core::clone::Clone::clone(&self.amount),
        }
    }
}
#[automatically_derived]
impl ::es_core::DynEvent for PaymentProcessed {
    fn name(&self) -> ::es_core::EventName<'static> {
        Self::NAME
    }
}
#[automatically_derived]
impl ::es_core::Event for PaymentProcessed {
    const NAME: ::es_core::EventName<'static> = ::es_core::EventName::new(
        "PaymentProcessed",
    );
}
#[automatically_derived]
impl ::es_core::Idempotent for PaymentProcessed
where
    PaymentId: std::fmt::Display,
{
    fn get_idempotency_key(
        &self,
    ) -> Result<::es_core::IdempotencyKey, ::es_core::IdempotencyKeyError> {
        let user_parts: Vec<String> = <[_]>::into_vec(
            ::alloc::boxed::box_new([self.payment.id.to_string()]),
        );
        ::es_core::IdempotencyKey::try_new(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}-{1}", "PaymentProcessed", user_parts.join("-")),
                )
            }),
        )
    }
}
#[automatically_derived]
impl ::es_core::Correlated for PaymentProcessed
where
    UserId: std::fmt::Display,
{
    fn get_correlation_id(
        &self,
    ) -> Result<::es_core::CorrelationId, ::es_core::CorrelationIdError> {
        let user_parts: Vec<String> = <[_]>::into_vec(
            ::alloc::boxed::box_new([self.user.id.to_string()]),
        );
        ::es_core::CorrelationId::try_new(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}-{1}", "PaymentProcessed", user_parts.join("-")),
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
