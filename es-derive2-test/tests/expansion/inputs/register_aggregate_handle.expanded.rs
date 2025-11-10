use es_interface::aggregate::{AggregateHandle, NamedAggregate};
use es_interface::HandlerOutput;
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
impl NamedAggregate for MyAggregate {
    const NAME: &'static str = "MyAggregate";
}
impl HandlerOutput for () {}
impl AggregateHandle<MyEvent> for MyAggregate {
    type Output = ();
    fn handle(&self, _event: MyEvent) -> Self::Output {}
}
