use es_interface::aggregate::{AggregateHandle, NamedAggregate};
use es_interface::HandlerOutput;

// Mock types for testing
#[derive(es_derive2::Event)]
pub struct MyEvent;

pub struct MyAggregate;

impl NamedAggregate for MyAggregate {
    const NAME: &'static str = "MyAggregate";
}

impl HandlerOutput for () {}

#[es_derive2::es_register]
impl AggregateHandle<MyEvent> for MyAggregate {
    type Output = ();
    fn handle(&self, _event: MyEvent) -> Self::Output {}
}
