use chrono::{DateTime, Utc};
use hktrading_client::types::SymbolTicker;
use std::{cell::RefCell, collections::HashMap};

use super::{Candles, OhlcOrderBlock, Resolution};
use hk_infra::{self, HkError};

pub trait OhlcFeedService {
    // fn get_ohlc_by_symbol_resolution(&self, symbol_ticker: &str, resolution: &str) -> Option<&Vec<Candles>>;
    fn get_ohlc_by_symbol_resolution_time_range(
        &mut self,
        symbol_ticker: SymbolTicker,
        resolution: Resolution,
        from_time: Option<DateTime<Utc>>,
        to_time: Option<DateTime<Utc>>,
        // ) -> HkBoxFuture<Result<Candles, HkError>>;
    ) -> impl futures::Future<Output = Result<Candles, HkError>>;
    // fn set_data(&mut self, symbol: &str, resolution: &str, candles: Vec<Candles>);
}

pub struct OhlcFeedServiceImpl {
    // one symbol ticker + resolution -> array of candles
    data: HashMap<SymbolTicker, HashMap<Resolution, OhlcOrderBlock>>,
    hkclient: hktrading_client::Client,
}

impl OhlcFeedServiceImpl {
    pub fn new(hkclient: hktrading_client::Client) -> Self {
        Self {
            data: HashMap::new(),
            hkclient: hkclient,
        }
    }

    // Get list of candles by symbol and resolution
    // pub fn get_ohlc_by_symbol_resolution(
    //     &self,
    //     symbol_ticker: SymbolTicker,
    //     resolution: Resolution,
    // ) -> Option<&Vec<Candles>> {
    //     self.data
    //         .get(&symbol_ticker)
    //         .and_then(|x| x.get(&resolution))
    // }

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

    pub fn set_data(
        &mut self,
        symbol: &SymbolTicker,
        resolution: Resolution,
        candles: Vec<Candles>,
    ) {
        // let symbol_data = self
        //     .data
        //     .entry(symbol)
        //     .or_insert(HashMap::new());
        // symbol_data.insert(resolution.to_string(), candles);
    }
}

impl OhlcFeedService for OhlcFeedServiceImpl {
    async fn get_ohlc_by_symbol_resolution_time_range(
        &mut self,
        symbol_ticker: SymbolTicker,
        resolution: Resolution,
        from_time: Option<DateTime<Utc>>,
        to_time: Option<DateTime<Utc>>,
        // ) -> HkBoxFuture<Result<Candles, HkError>> {
        // ) -> impl futures::Future<Output = Result<Candles, HkError>> {
    ) -> Result<Candles, HkError> {
        // find the not found data in data
        // if not found, call the hkclient to get the data
        let r = self
            .hkclient
            .get_candles(
                from_time.map(|t| t.timestamp()),
                (resolution).into(),
                symbol_ticker,
                to_time.map(|t| t.timestamp()),
            )
            .await;
        // let candleResp = r.unwrap();
        // candleResp.into_inner().into()
        let cr: Result<Candles, hk_infra::HkError> = match r {
            Ok(candle_resp) => {
                // self.set_data(symbol, resolution, candles);
                Ok(candle_resp.into_inner().into())
            }
            Err(e) => Err(e.into()),
        };
        let candles = cr?;
        
        // after getting the data, set the data to store
        let candle_block = self.data
            .entry(symbol_ticker)
            .or_insert(HashMap::new())
            .entry(resolution)
            .or_insert(OhlcOrderBlock::new());
        candle_block.merge_block(RefCell::new(Box::new(candles.clone())))?;

        Ok(candles)
    }
}
