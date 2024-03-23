pub type Result<T> = std::result::Result<T, HkError>;

#[derive(Debug)]
pub enum HkError {
    InvalidParameter,
    CsvError(csv::Error),
    CsvMissingColumn(String),
    HkTradingError(hktrading_client::types::HkError),
    HkDataError(String),
    UnknownError(String),
}

impl From<csv::Error> for HkError {
    fn from(e: csv::Error) -> Self {
        HkError::CsvError(e)
    }
}
