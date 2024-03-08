use chrono::{DateTime, Utc};
use csv::{Position, StringRecord};

use crate::errors::TaError;

use super::{Candle, CandleDataSource, Candles, DataSourceMeta, Resolution};

/**
 * CSV Definition
 */

pub struct CandleCSVLoadOption {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

pub struct CSVFileMetadata {
    file_path: String,
    resolution: Resolution,
    ticker: String,
}

pub struct CandleCSVDataSource {}

// Read Data from CSV

impl CandleCSVDataSource {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load_csv_file(
        file_path: &str,
        load_option: Option<CandleCSVLoadOption>,
    ) -> Result<(CSVFileMetadata, Candles), TaError> {
        let mut reader = csv::Reader::from_path(file_path).map_err(|e| TaError::CsvError(e))?;

        let mut candles = Candles::new();
        let mut csv_file_metadata: CSVFileMetadata = CSVFileMetadata {
            file_path: file_path.to_string(),
            resolution: Resolution::M1,
            ticker: "XAUUSD".to_string(),
        };

        let load_option = load_option.unwrap_or(CandleCSVLoadOption {
            offset: None,
            limit: None,
        });
        let offset = load_option.offset.unwrap_or(0);
        let limit = load_option.limit.unwrap_or(-1);

        if offset > 0 {
            let mut position = Position::new();
            position.set_line(offset as u64);
            reader.seek(position).unwrap();
        }

        for result in reader.records() {
            let record = result.map_err(|e| TaError::CsvError(e))?;
            // record.
            let candle: Candle = self.parse_string_record(record)?;
            candles.push_candle(&candle);

            if limit > 0 && candles.open_time.len() as i64 >= limit {
                break;
            }
        }

        if candles.open_time.len() > 2 {
            let from_time = candles.open_time[0];
            let to_time = candles.open_time[1];
            let diff_seconds = (to_time - from_time).num_seconds();
            Resolution::from_seconds(diff_seconds);
        }

        Ok((csv_file_metadata, candles))
    }

    #[inline]
    fn parse_string_record(&self, record: StringRecord) -> Result<Candle, TaError> {
        let csv_missing_row = || TaError::CsvMissingColumn(format!("missing in {:?}", record));

        // Parse each column
        let date = record.get(1).ok_or_else(csv_missing_row)?;
        let time = record.get(2).ok_or_else(csv_missing_row)?;
        let open = record.get(3).ok_or_else(csv_missing_row)?;
        let high = record.get(4).ok_or_else(csv_missing_row)?;
        let low = record.get(5).ok_or_else(csv_missing_row)?;
        let close = record.get(6).ok_or_else(csv_missing_row)?;
        let vol = record.get(7).ok_or_else(csv_missing_row)?;

        // Convert two columns date and time to datetime
        let datetime = format!("{} {} +00:00", date, time);
        let datetime_from_string = DateTime::parse_from_str(&datetime, "%Y%m%d %H%M%S %z")
            .map_err(|e| {
                TaError::CsvMissingColumn(format!(
                    "parsing date time failing in \"{}\", {:?}, {}",
                    datetime, record, e
                ))
            })?
            .with_timezone(&Utc);

        // Create candle from parsed columns
        let candle = Candle {
            open_time: datetime_from_string,
            // time: time.to_string(),
            open: open.parse::<f64>().unwrap(),
            high: high.parse::<f64>().unwrap(),
            low: low.parse::<f64>().unwrap(),
            close: close.parse::<f64>().unwrap(),
            volume: vol.parse::<f64>().ok(),
            trade_count: None,
        };
        Ok(candle)
    }
}

impl CandleDataSource for CandleCSVDataSource {
    fn get_metadata(self) -> Vec<DataSourceMeta> {
        todo!()
    }

    fn get_data_source_from<'a>(&self, option: super::DataSourceGet) -> super::Candles {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use csv::Reader;

    use crate::data::Candle;

    use super::CandleCSVDataSource;

    #[test]
    fn test_load_csv() {
        let data = "\
ticker,date,time,open,high,low,close,vol
XAUUSD,20220608,000000,1849,1849.1,1849,1849,4
XAUUSD,20220608,000100,1849.1,1849.3,1849,1849.3,4
XAUUSD,20220608,000200,1849.4,1849.4,1849.2,1849.3,4
";
        let mut rdr = Reader::from_reader(data.as_bytes());
        // CandleCSVDataSource::load_csv();
        let ds = CandleCSVDataSource::new();
        let mut candles: Vec<Candle> = vec![];
        for record in rdr.records() {
            let record = record.unwrap();
            let candle = ds.parse_string_record(record).unwrap();
            println!("{:?}", candle);
            candles.push(candle);
        }
        assert_eq!(candles.len(), 3);
        assert_eq!(
            candles[0],
            Candle {
                open_time: "2022-06-08T00:00:00+00:00".parse().unwrap(),
                open: 1849.0,
                high: 1849.1,
                low: 1849.0,
                close: 1849.0,
                volume: Some(4.0),
                trade_count: None,
            }
        );
        assert_eq!(
            candles[1],
            Candle {
                open_time: "2022-06-08T00:01:00+00:00".parse().unwrap(),
                open: 1849.1,
                high: 1849.3,
                low: 1849.0,
                close: 1849.3,
                volume: Some(4.0),
                trade_count: None,
            }
        );
    }
}
