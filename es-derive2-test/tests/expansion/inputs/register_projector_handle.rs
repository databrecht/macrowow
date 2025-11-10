use es_interface::projector::{NamedProjector, ProjectorHandle};
use es_interface::{HandlerOutput, Report};

// Mock types for testing
#[derive(es_derive2::Event)]
pub struct MyEvent;

pub struct MyProjector;

impl NamedProjector for MyProjector {
    const NAME: &'static str = "MyProjector";
}

impl HandlerOutput for () {}

#[es_derive2::es_register]
impl ProjectorHandle<MyEvent> for MyProjector {
    type Output = ();

    async fn handle(&self, _event: MyEvent) -> Result<Self::Output, Report> {
        Ok(())
    }
}
