pub trait EventSourcedAggregate<E, Err>: Sized {
    /// Create a new aggregate instance from the first event in the event stream.
    ///
    /// This method is called only once, to initialize the aggregate with its first event,
    /// typically something like `ComponentRegistered` or `CollectionCreated`.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided event is not a valid initializer
    /// (e.g. an update event applied to an uninitialized aggregate).
    fn from_initial_event(event: &E) -> Result<Self, Err>;

    /// Apply a single domain event to mutate the aggregate state.
    ///
    /// # Errors
    ///
    /// Returns an error if the event is not applicable to the current state.
    fn apply(&mut self, event: &E) -> Result<(), Err>;

    /// Reconstruct an aggregate by applying a sequence of events.
    ///
    /// # Errors
    ///
    /// Returns an error if the first event is invalid to initialize the aggregate,
    /// or if any subsequent event cannot be applied.
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
