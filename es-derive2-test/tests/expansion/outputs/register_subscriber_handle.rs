impl SubscriberHandle<MyEvent> for MySubscriber {
    type Output = ();
    async fn handle(&self, _event: MyEvent) -> Result<Self::Output, Report> {
        Ok(())
    }
}