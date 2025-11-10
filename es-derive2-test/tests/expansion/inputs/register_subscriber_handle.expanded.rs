use es_interface::subscriber::{NamedSubscriber, SubscriberHandle};
use es_interface::{HandlerOutput, Report};
pub struct MyEvent;
pub struct MySubscriber;
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
impl NamedSubscriber for MySubscriber {
    const NAME: &'static str = "MySubscriber";
}
impl HandlerOutput for () {}
impl SubscriberHandle<MyEvent> for MySubscriber {
    type Output = ();
    async fn handle(&self, _event: MyEvent) -> Result<Self::Output, Report> {
        Ok(())
    }
}
