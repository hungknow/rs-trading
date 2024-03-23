use std::collections::HashMap;

use super::Candles;

pub struct OhlcFeedService {
    // one symbol ticker + resolution -> array of candles
    data: HashMap<String, HashMap<String, Vec<Candles>>>,
}

impl OhlcFeedService {
    pub fn new() -> Self {
        OhlcFeedService {
            data: HashMap::new(),
        }
    }

    // Get list of candles by symbol and resolution
    pub fn get_ohlc_by_symbol_resolution(&self, symbol_ticker: &str, resolution: &str) -> Option<&Vec<Candles>> {
        self.data.get(symbol_ticker).and_then(|x| x.get(resolution))
    }

    // Get candles by symbol, resolution and time range, totime is optional
    // pub fn get_ohlc_by_symbol_resolution_time_range(&self, symbol_ticker: &str, resolution: &str, from_time: i64, to_time: Option<i64>) -> Option<Vec<Candles>> {
    //     let candles = self.get_ohlc_by_symbol_resolution(symbol_ticker, resolution);
    //     if candles.is_none() {
    //         return None;
    //     }
    //     let candles = candles.unwrap();
    //     let mut result = Vec::new();
    //     for candle in candles {
    //         if candle.open_time >= from_time {
    //             if let Some(to_time) = to_time {
    //                 if candle.open_time <= to_time {
    //                     result.push(candle.clone());
    //                 }
    //             } else {
    //                 result.push(candle.clone());
    //             }
    //         }
    //     }
    //     Some(result)
    // }
    

    pub fn set_data(&mut self, symbol: &str, resolution: &str, candles: Vec<Candles>) {
        let symbol_data = self.data.entry(symbol.to_string()).or_insert(HashMap::new());
        symbol_data.insert(resolution.to_string(), candles);
    }
}