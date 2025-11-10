use crate::{event::Event, HandlerOutput};

/// Trait for applying events to aggregates (event sourcing)
pub trait AggregateApply<E: Event> {
    fn apply(&mut self, event: &E);
}

/// Trait for handling commands on aggregates
pub trait AggregateHandle<E: Event>: NamedAggregate {
    type Output: HandlerOutput;

    /// Emit [`Output`](Self) from a given input [`Event`] `E`.
    fn handle(&self, event: E) -> Self::Output;
}

/// Trait for named aggregates
pub trait NamedAggregate {
    const NAME: &'static str;
}
