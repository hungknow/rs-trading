use hk_trading::data::Candles;

pub struct CandlesState {
    candles: Candles,
}

pub async fn load_candles(
    symbol: String,
    interval: String,
    start: i64,
    end: i64,
) -> Result<Candles, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&interval={}&startTime={}&endTime={}",
        symbol, interval, start, end
    );
    // let resp = reqwest::get(&url).await?.json::<Vec<Vec<Value>>>().await?;
    // let candles = resp
    //     .iter()
    //     .map(|c| Candle {
    //         open_time: c[0].as_i64().unwrap(),
    //         open: c[1].as_str().unwrap().parse().unwrap(),
    //         high: c[2].as_str().unwrap().parse().unwrap(),
    //         low: c[3].as_str().unwrap().parse().unwrap(),
    //         close: c[4].as_str().unwrap().parse().unwrap(),
    //         volume: c[5].as_str().unwrap().parse().unwrap(),
    //         close_time: c[6].as_i64().unwrap(),
    //         quote_asset_volume: c[7].as_str().unwrap().parse().unwrap(),
    //         number_of_trades: c[8].as_i64().unwrap(),
    //         taker_buy_base_asset_volume: c[9].as_str().unwrap().parse().unwrap(),
    //         taker_buy_quote_asset_volume: c[10].as_str().unwrap().parse().unwrap(),
    //     })
    //     .collect();
    Ok(())
}