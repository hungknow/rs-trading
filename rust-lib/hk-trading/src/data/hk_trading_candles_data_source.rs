use std::sync::Arc;

use crate::errors::HkError;
use chrono::{NaiveDateTime, TimeZone, Utc};
use hktrading_client::types::SymbolTicker;

use super::CandleDataSource;

struct HkTradingCandleDataSource {
    hk_trading_client: Arc<hktrading_client::Client>,
}

impl HkTradingCandleDataSource {
    pub fn new(client: Arc<hktrading_client::Client>) -> Self {
        Self {
            hk_trading_client: client,
        }
    }
}

// #[async_trait]
// impl CandleDataSource for HkTradingCandleDataSource {
impl HkTradingCandleDataSource {
    fn get_metadata(self) -> Vec<super::DataSourceMeta> {
        todo!()
    }

    async fn get_data_source_from<'a>(
        &self,
        option: super::DataSourceGet<'a>,
    ) -> Result<super::Candles, HkError> {
        let candles_response = self
            .hk_trading_client
            .get_candles(
                Some(option.start_time.timestamp()),
                option.resolution.into(),
                SymbolTicker::MockXauusd,
                option.end_time.map(|t| t.timestamp()),
            )
            .await
            .map_err(|e| e.into());
        match candles_response {
            Ok(candles) => Ok(candles.into_inner().into()),
            Err(e) => Err(e),
        }
    }
}

impl From<hktrading_client::types::Candles> for super::Candles {
    fn from(candles: hktrading_client::types::Candles) -> super::Candles {
        let mut c = super::Candles::new();
        c.set_volumes(candles.vols.iter().map(|v| Some(*v as f64)).collect())
            .set_open_times(
                candles
                    .times
                    .iter()
                    .map(|t| {
                        let dt: NaiveDateTime = NaiveDateTime::from_timestamp_opt(*t, 0).unwrap();
                        Utc.from_utc_datetime(&dt)
                    })
                    .collect(),
            );
        c
    }
}

impl From<crate::data::Resolution> for hktrading_client::types::Resolution {
    fn from(resolution: crate::data::Resolution) -> Self {
        match resolution {
            crate::data::Resolution::M1 => hktrading_client::types::Resolution::M1,
            crate::data::Resolution::M5 => hktrading_client::types::Resolution::M5,
            crate::data::Resolution::M15 => hktrading_client::types::Resolution::M15,
            crate::data::Resolution::M30 => hktrading_client::types::Resolution::M30,
            crate::data::Resolution::H1 => hktrading_client::types::Resolution::H1,
            crate::data::Resolution::H4 => hktrading_client::types::Resolution::H4,
            crate::data::Resolution::D1 => hktrading_client::types::Resolution::D1,
            crate::data::Resolution::W1 => hktrading_client::types::Resolution::W1,
        }
    }
}

impl From<hktrading_client::Error<hktrading_client::types::HkError>> for HkError {
    fn from(e: hktrading_client::Error<hktrading_client::types::HkError>) -> Self {
        match e {
            hktrading_client::Error::InvalidRequest(e) => HkError::UnknownError(e),
            hktrading_client::Error::CommunicationError(e) => HkError::UnknownError(e.to_string()),
            hktrading_client::Error::InvalidUpgrade(e) => HkError::UnknownError(e.to_string()),
            hktrading_client::Error::ErrorResponse(e) => {
                HkError::HkServerError(e.code.unwrap(), e.message.as_deref().unwrap().to_string())
            }
            hktrading_client::Error::ResponseBodyError(e) => HkError::UnknownError(e.to_string()),
            hktrading_client::Error::InvalidResponsePayload(_, e) => {
                HkError::UnknownError(e.to_string())
            }
            hktrading_client::Error::UnexpectedResponse(e) => {
                HkError::UnknownError(e.status().to_string())
            }
            hktrading_client::Error::PreHookError(e) => HkError::UnknownError(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::{Duration, NaiveDateTime, TimeZone, Utc};

    use crate::data::DataSourceGet;

    use super::HkTradingCandleDataSource;

    #[tokio::test]
    async fn test_get_data_source_from() {
        let hk_client = Arc::new(hktrading_client::Client::new("http://localhost:9001"));
        let client = HkTradingCandleDataSource::new(hk_client);
        let native_time = NaiveDateTime::from_timestamp_opt(1654646400, 0).unwrap();
        let dt = Utc.from_utc_datetime(&native_time);
        let candles = client
            .get_data_source_from(DataSourceGet {
                start_time: dt,
                resolution: crate::data::Resolution::M1,
                symbol: "", //hktrading_client::types::SymbolTicker::MockXauusd,
                end_time: Some(dt + Duration::seconds(120)),
            })
            .await
            .unwrap();
        assert_eq!(candles.open_times.len(), 2);
        // The first candle should be at 1654646400
        assert_eq!(candles.open_times[0].timestamp(), 1654646400);
    }
}
