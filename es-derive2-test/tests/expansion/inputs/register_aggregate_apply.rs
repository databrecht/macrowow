use es_interface::aggregate::AggregateApply;

// Mock types for testing
#[derive(es_derive2::Event)]
pub struct MyEvent;

pub struct MyAggregate;

#[es_derive2::es_register]
impl AggregateApply<MyEvent> for MyAggregate {
    fn apply(&mut self, _event: &MyEvent) {}
}
