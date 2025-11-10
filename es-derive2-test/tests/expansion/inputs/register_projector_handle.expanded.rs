use es_interface::projector::{NamedProjector, ProjectorHandle};
use es_interface::{HandlerOutput, Report};
pub struct MyEvent;
pub struct MyProjector;
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
impl NamedProjector for MyProjector {
    const NAME: &'static str = "MyProjector";
}
impl HandlerOutput for () {}
impl ProjectorHandle<MyEvent> for MyProjector {
    type Output = ();
    async fn handle(&self, _event: MyEvent) -> Result<Self::Output, Report> {
        Ok(())
    }
}
