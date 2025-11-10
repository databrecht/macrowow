use crate::{event::Event, HandlerOutput, Report};
use std::future::Future;

/// Trait for projectors that handle events
pub trait ProjectorHandle<E: Event>: NamedProjector {
    type Output: HandlerOutput;

    fn handle(
        &self,
        event: E,
    ) -> impl Future<Output = Result<Self::Output, Report>> + Send;
}

/// Trait for named projectors
pub trait NamedProjector {
    const NAME: &'static str;
}
