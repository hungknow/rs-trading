# rs-trade
All libraries that useful for trading written in Rust

- rs-data: Data generator govern the data feed event. The purpose of event is to notify if there's fetched data, wait for incoming data or the no more data.
- rs-strategy: The

# Use cases
- [ ] Receive data source
    - [ ] Data source can be the websocket stream (binance,...) or historical stream
- [ ] Write the strategy

# Getting started

## Run examples

```
jupyter notebook
```

```
cargo run --example historical_candle
```

## Data handler
Define `MarketEvent`, and provides MarketGenerator trait for handling the generation of them.

```rs
let mut marketFeed = historical::MarketFeed::new(
    candle_json.into_iter()
)

loop {
    let data = match marketFeed.next() {
        Feed::Next(market_event) => market_event,
        Feed::Finished => break,
        Feed::Unhealthy => continue,
    }
}
```

## Strategy

A strategy can send, modify and cancel buy/sell orders.

```rs
let market_event = market_event_trade(Side::Buy);

let mut strategy = RSIStrategy::new(StrategyConfig {
    rsi_period: 14,
});

let signal_event = strategy.generate_signal(&market_event);
```

- Apply any desired indicators
- Define the conditions under which they want to buy or sell, as well as set any other parameters such as stop-loss and take-profit levels.


## Portfolio

```rs

```

## Execution

## Statistic

## Engine & Trader