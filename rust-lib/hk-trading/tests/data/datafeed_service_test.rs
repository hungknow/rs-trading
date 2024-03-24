use std::f32::consts::E;

use chrono::DateTime;
use hk_trading::data::{
    datafeed_service::{OhlcFeedService, OhlcFeedServiceImpl},
    Resolution,
};
use hktrading_client::types::SymbolTicker;

#[tokio::test]
async fn test_ohlc_feed_service() {
    let mut client = hktrading_client::Client::new("http://localhost:9001");
    let mut ohldFeedService = OhlcFeedServiceImpl::new(client);
    let from_time = DateTime::from_timestamp(10000, 0);
    match ohldFeedService
        .get_ohlc_by_symbol_resolution_time_range(
            SymbolTicker::MockXauusd,
            Resolution::M1,
            from_time,
            // Let the server to calculate the to_time
            None,
        )
        .await
    {
        Ok(candles) => {
            assert_eq!(candles.open_times.len(), 1500);
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}
