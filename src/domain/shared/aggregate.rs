pub trait EventSourcedAggregate<E, Err>: Sized {
    /// Try to create the aggregate from the initial event
    fn from_initial_event(event: &E) -> Result<Self, Err>;

    /// Apply a subsequent event to mutate the state
    fn apply(&mut self, event: &E) -> Result<(), Err>;

    /// Rehydrate the aggregate from a sequence of events
    fn rehydrate(events: &[E]) -> Result<Self, Err> {
        let (first, rest) = events
            .split_first()
            .ok_or_else(|| Self::invalid_initial_event())?;

        let mut agg = Self::from_initial_event(first)?;

        for e in rest {
            agg.apply(e)?;
        }

        Ok(agg)
    }

    /// Optional fallback error if the first event is missing
    fn invalid_initial_event() -> Err;
}
