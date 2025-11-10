use es_interface::subscriber::{NamedSubscriber, SubscriberHandle};
use es_interface::{HandlerOutput, Report};

// Mock types for testing
#[derive(es_derive2::Event)]
pub struct MyEvent;

pub struct MySubscriber;

impl NamedSubscriber for MySubscriber {
    const NAME: &'static str = "MySubscriber";
}

impl HandlerOutput for () {}

#[es_derive2::es_register]
impl SubscriberHandle<MyEvent> for MySubscriber {
    type Output = ();

    async fn handle(&self, _event: MyEvent) -> Result<Self::Output, Report> {
        Ok(())
    }
}
