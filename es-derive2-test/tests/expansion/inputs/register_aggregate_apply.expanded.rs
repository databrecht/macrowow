use es_interface::aggregate::AggregateApply;
pub struct MyEvent;
pub struct MyAggregate;
impl ::es_interface::DynEvent for MyEvent {
    fn name(&self) -> ::es_interface::EventName<'static> {
        Self::NAME
    }
}
impl ::es_interface::Event for MyEvent {
    const NAME: ::es_interface::EventName<'static> = ::es_interface::EventName::new(
        "MyEvent",
    );
}
impl ::es_interface::event::NamedEvent for MyEvent {
    const NAME: ::es_interface::EventName<'static> = ::es_interface::EventName::new(
        "MyEvent",
    );
}
impl AggregateApply<MyEvent> for MyAggregate {
    fn apply(&mut self, _event: &MyEvent) {}
}
