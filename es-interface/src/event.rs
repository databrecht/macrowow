use crate::EventName;

/// Trait for named events
pub trait Event: Sized {
    const NAME: EventName<'static>;
}
