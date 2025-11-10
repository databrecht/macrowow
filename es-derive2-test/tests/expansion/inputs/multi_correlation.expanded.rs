use es_interface::*;
/// Event with multiple correlation fields to test combination behavior
pub struct MultiCorrelationEvent {
    #[es(correlation)]
    pub user_id: String,
    #[es(correlation)]
    pub session_id: String,
    pub data: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for MultiCorrelationEvent {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "MultiCorrelationEvent",
            "user_id",
            &self.user_id,
            "session_id",
            &self.session_id,
            "data",
            &&self.data,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for MultiCorrelationEvent {
    #[inline]
    fn clone(&self) -> MultiCorrelationEvent {
        MultiCorrelationEvent {
            user_id: ::core::clone::Clone::clone(&self.user_id),
            session_id: ::core::clone::Clone::clone(&self.session_id),
            data: ::core::clone::Clone::clone(&self.data),
        }
    }
}
#[automatically_derived]
impl ::es_interface::DynEvent for MultiCorrelationEvent {
    fn name(&self) -> ::es_interface::EventName<'static> {
        Self::NAME
    }
}
#[automatically_derived]
impl ::es_interface::Event for MultiCorrelationEvent {
    const NAME: ::es_interface::EventName<'static> = ::es_interface::EventName::new(
        "MultiCorrelationEvent",
    );
}
#[automatically_derived]
impl ::es_interface::Correlated for MultiCorrelationEvent
where
    String: std::fmt::Display,
    String: std::fmt::Display,
{
    fn get_correlation_id(
        &self,
    ) -> Result<::es_interface::CorrelationId, ::es_interface::CorrelationIdError> {
        let user_parts: Vec<String> = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                self.session_id.to_string(),
                self.user_id.to_string(),
            ]),
        );
        ::es_interface::CorrelationId::try_new(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "{0}-{1}", "MultiCorrelationEvent", user_parts.join("-"),
                    ),
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
