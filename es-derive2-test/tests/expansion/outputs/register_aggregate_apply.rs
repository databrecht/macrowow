impl AggregateApply<MyEvent> for MyAggregate {
    fn apply(&mut self, _event: &MyEvent) {}
}