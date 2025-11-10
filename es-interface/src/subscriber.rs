use crate::{event::Event, HandlerOutput, Report};
use std::future::Future;

/// Trait for subscribers that handle events
pub trait SubscriberHandle<E: Event>: NamedSubscriber {
    type Output: HandlerOutput;

    fn handle(
        &self,
        event: E,
    ) -> impl Future<Output = Result<Self::Output, Report>> + Send;
}

/// Trait for named subscribers
pub trait NamedSubscriber {
    const NAME: &'static str;
}
