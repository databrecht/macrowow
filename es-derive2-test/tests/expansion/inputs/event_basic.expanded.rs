use es_interface::*;
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
/// Event demonstrating direct field access (simpler test first)
pub struct PaymentProcessed {
    pub payment: PaymentId,
    pub user: UserId,
    pub amount: u64,
    #[es(idempotency)]
    pub payment_id: String,
    #[es(correlation)]
    pub user_id: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for PaymentProcessed {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "PaymentProcessed",
            "payment",
            &self.payment,
            "user",
            &self.user,
            "amount",
            &self.amount,
            "payment_id",
            &self.payment_id,
            "user_id",
            &&self.user_id,
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
            payment_id: ::core::clone::Clone::clone(&self.payment_id),
            user_id: ::core::clone::Clone::clone(&self.user_id),
        }
    }
}
#[automatically_derived]
impl ::es_interface::DynEvent for PaymentProcessed {
    fn name(&self) -> ::es_interface::EventName<'static> {
        Self::NAME
    }
}
#[automatically_derived]
impl ::es_interface::Event for PaymentProcessed {
    const NAME: ::es_interface::EventName<'static> = ::es_interface::EventName::new(
        "PaymentProcessed",
    );
}
#[automatically_derived]
impl ::es_interface::Idempotent for PaymentProcessed
where
    String: std::fmt::Display,
{
    fn get_idempotency_key(
        &self,
    ) -> Result<::es_interface::IdempotencyKey, ::es_interface::IdempotencyKeyError> {
        let user_parts: Vec<String> = <[_]>::into_vec(
            ::alloc::boxed::box_new([self.payment_id.to_string()]),
        );
        ::es_interface::IdempotencyKey::try_new(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}-{1}", "PaymentProcessed", user_parts.join("-")),
                )
            }),
        )
    }
}
#[automatically_derived]
impl ::es_interface::Correlated for PaymentProcessed
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
                    format_args!("{0}-{1}", "PaymentProcessed", user_parts.join("-")),
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
