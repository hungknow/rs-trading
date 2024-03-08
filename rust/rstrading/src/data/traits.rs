
/// Communicates the state of the [`Feed`] as well as the next event.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Feed<Event> {
    Next(Event),
    Unhealthy,
    Finished
}

pub trait MarketGenerator<Event> {
    /// Return the next market `Event`.
    fn next(&mut self) -> Feed<Event>;
}