impl AggregateHandle<MyEvent> for MyAggregate {
    type Output = ();
    fn handle(&self, _event: MyEvent) -> Self::Output {}
}