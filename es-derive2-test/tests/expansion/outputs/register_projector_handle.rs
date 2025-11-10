impl ProjectorHandle<MyEvent> for MyProjector {
    type Output = ();
    async fn handle(&self, _event: MyEvent) -> Result<Self::Output, Report> {
        Ok(())
    }
}