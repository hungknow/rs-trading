use super::traits::{Feed, MarketGenerator};

pub struct MarketFeed<Iter, Event>
where
    Iter: Iterator<Item = Event>,
{
    pub market_iterator: Iter,
}

impl<Iter, Event> MarketGenerator<Event> for MarketFeed<Iter, Event>
where
    Iter: Iterator<Item = Event>,
{
    fn next(&mut self) -> super::traits::Feed<Event> {
        self.market_iterator
            .next()
            .map_or(Feed::Finished, Feed::Next)
    }
}

impl<Iter, Event> MarketFeed<Iter, Event>
where
    Iter: Iterator<Item = Event>,
{
    pub fn new<IntoIter>(market_iterator: IntoIter) -> Self
    where
        IntoIter: IntoIterator<Item = Event, IntoIter = Iter>,
    {
        Self {
            market_iterator: market_iterator.into_iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::traits::{MarketGenerator, Feed};

    use super::MarketFeed;

    #[test]
    fn test_next() {
        let someValue = [10, 20];
        let mut marketFeed = MarketFeed::new(someValue);
        assert_eq!(marketFeed.next(), Feed::Next(someValue[0]));
        assert_eq!(marketFeed.next(), Feed::Next(someValue[1]));
    }
}
