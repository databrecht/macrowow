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
pub struct TransactionRef {
    pub ref_num: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for TransactionRef {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "TransactionRef",
            "ref_num",
            &&self.ref_num,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TransactionRef {
    #[inline]
    fn clone(&self) -> TransactionRef {
        TransactionRef {
            ref_num: ::core::clone::Clone::clone(&self.ref_num),
        }
    }
}
#[es(awaits(TransferRequested, PaymentProcessed))]
#[es(idempotency(payment.id, transaction.ref_num))]
#[es(correlation(user.id, status = "exists"))]
pub struct PaymentAuthorizationReleaseRequestDispatched {
    pub payment: PaymentId,
    pub user: UserId,
    pub transaction: TransactionRef,
    pub amount: u64,
}
#[automatically_derived]
impl ::core::fmt::Debug for PaymentAuthorizationReleaseRequestDispatched {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "PaymentAuthorizationReleaseRequestDispatched",
            "payment",
            &self.payment,
            "user",
            &self.user,
            "transaction",
            &self.transaction,
            "amount",
            &&self.amount,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for PaymentAuthorizationReleaseRequestDispatched {
    #[inline]
    fn clone(&self) -> PaymentAuthorizationReleaseRequestDispatched {
        PaymentAuthorizationReleaseRequestDispatched {
            payment: ::core::clone::Clone::clone(&self.payment),
            user: ::core::clone::Clone::clone(&self.user),
            transaction: ::core::clone::Clone::clone(&self.transaction),
            amount: ::core::clone::Clone::clone(&self.amount),
        }
    }
}
#[automatically_derived]
impl ::es_core::DynEvent for PaymentAuthorizationReleaseRequestDispatched {
    fn name(&self) -> ::es_core::EventName<'static> {
        Self::NAME
    }
}
#[automatically_derived]
impl ::es_core::Event for PaymentAuthorizationReleaseRequestDispatched {
    const NAME: ::es_core::EventName<'static> = ::es_core::EventName::new(
        "PaymentAuthorizationReleaseRequestDispatched",
    );
}
#[automatically_derived]
impl ::es_core::ExpectsAwaitedSet<TransferRequested>
for PaymentAuthorizationReleaseRequestDispatched {}
impl ::es_core::ExpectsAwaitedSet<PaymentProcessed>
for PaymentAuthorizationReleaseRequestDispatched {}
#[automatically_derived]
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
#[automatically_derived]
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
pub struct TransferRequestedResponse {
    pub status: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for TransferRequestedResponse {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "TransferRequestedResponse",
            "status",
            &&self.status,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TransferRequestedResponse {
    #[inline]
    fn clone(&self) -> TransferRequestedResponse {
        TransferRequestedResponse {
            status: ::core::clone::Clone::clone(&self.status),
        }
    }
}
#[automatically_derived]
impl ::es_core::DynEvent for TransferRequestedResponse {
    fn name(&self) -> ::es_core::EventName<'static> {
        Self::NAME
    }
}
#[automatically_derived]
impl ::es_core::Event for TransferRequestedResponse {
    const NAME: ::es_core::EventName<'static> = ::es_core::EventName::new(
        "TransferRequestedResponse",
    );
}
pub enum TransferRequested {
    Response(TransferRequestedResponse),
}
#[automatically_derived]
impl ::core::fmt::Debug for TransferRequested {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            TransferRequested::Response(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Response",
                    &__self_0,
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TransferRequested {
    #[inline]
    fn clone(&self) -> TransferRequested {
        match self {
            TransferRequested::Response(__self_0) => {
                TransferRequested::Response(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
impl AwaitedSet for TransferRequested {
    const NAME: EventSetName = EventSetName::new("TransferRequested");
    const AWAITABLE_EVENTS: &'static [EventName<'static>] = &[
        <TransferRequestedResponse>::NAME,
    ];
    fn event_name(&self) -> EventName<'static> {
        match self {
            Self::Response(e) => e.name(),
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
        if event_name == <TransferRequestedResponse>::NAME {
            return envelope
                .downcast::<TransferRequestedResponse>()
                .map(Self::Response)
                .map_err(|_| downcast_error(event_name, "TransferRequestedResponse"));
        }
        Err(
            Report::msg(
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "Event \'{0}\' is not part of {1}. Expected one of: {2:?}",
                            event_name.as_str(), "TransferRequested",
                            Self::AWAITABLE_EVENTS,
                        ),
                    )
                }),
            ),
        )
    }
}
impl HasEventSet<TransferRequested> for TransferRequestedResponse {
    fn into_set(self) -> TransferRequested {
        TransferRequested::Response(self)
    }
    fn try_from_set(set: TransferRequested) -> Result<Self, TransferRequested> {
        match set {
            TransferRequested::Response(e) => Ok(e),
        }
    }
}
pub struct PaymentProcessedResponse {
    pub status: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for PaymentProcessedResponse {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "PaymentProcessedResponse",
            "status",
            &&self.status,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for PaymentProcessedResponse {
    #[inline]
    fn clone(&self) -> PaymentProcessedResponse {
        PaymentProcessedResponse {
            status: ::core::clone::Clone::clone(&self.status),
        }
    }
}
#[automatically_derived]
impl ::es_core::DynEvent for PaymentProcessedResponse {
    fn name(&self) -> ::es_core::EventName<'static> {
        Self::NAME
    }
}
#[automatically_derived]
impl ::es_core::Event for PaymentProcessedResponse {
    const NAME: ::es_core::EventName<'static> = ::es_core::EventName::new(
        "PaymentProcessedResponse",
    );
}
pub enum PaymentProcessed {
    Response(PaymentProcessedResponse),
}
#[automatically_derived]
impl ::core::fmt::Debug for PaymentProcessed {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            PaymentProcessed::Response(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Response",
                    &__self_0,
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for PaymentProcessed {
    #[inline]
    fn clone(&self) -> PaymentProcessed {
        match self {
            PaymentProcessed::Response(__self_0) => {
                PaymentProcessed::Response(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
impl AwaitedSet for PaymentProcessed {
    const NAME: EventSetName = EventSetName::new("PaymentProcessed");
    const AWAITABLE_EVENTS: &'static [EventName<'static>] = &[
        <PaymentProcessedResponse>::NAME,
    ];
    fn event_name(&self) -> EventName<'static> {
        match self {
            Self::Response(e) => e.name(),
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
        if event_name == <PaymentProcessedResponse>::NAME {
            return envelope
                .downcast::<PaymentProcessedResponse>()
                .map(Self::Response)
                .map_err(|_| downcast_error(event_name, "PaymentProcessedResponse"));
        }
        Err(
            Report::msg(
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "Event \'{0}\' is not part of {1}. Expected one of: {2:?}",
                            event_name.as_str(), "PaymentProcessed",
                            Self::AWAITABLE_EVENTS,
                        ),
                    )
                }),
            ),
        )
    }
}
impl HasEventSet<PaymentProcessed> for PaymentProcessedResponse {
    fn into_set(self) -> PaymentProcessed {
        PaymentProcessed::Response(self)
    }
    fn try_from_set(set: PaymentProcessed) -> Result<Self, PaymentProcessed> {
        match set {
            PaymentProcessed::Response(e) => Ok(e),
        }
    }
}
