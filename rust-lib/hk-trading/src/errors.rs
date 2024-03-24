use std::f32::consts::E;

pub type Result<T> = std::result::Result<T, HkError>;

#[derive(Debug)]
pub enum HkError {
    UnknownError(String),
    InvalidParameter,
    CsvError(csv::Error),
    CsvMissingColumn(String),
    HkDataError(String),
    HkServerError(i32, String),
}

impl From<csv::Error> for HkError {
    fn from(e: csv::Error) -> Self {
        HkError::CsvError(e)
    }
}

impl std::fmt::Display for HkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HkError::UnknownError(msg) => write!(f, "{}", msg),
            HkError::InvalidParameter => write!(f, "Invalid Param"),
            HkError::CsvError(e) => { write!(f, "{}", e) }
            HkError::CsvMissingColumn(e) => { write!(f, "{}", e) }
            HkError::HkDataError(e) => { write!(f, "{}", e) }
            HkError::HkServerError(code, msg) => { write!(f, "Server Error: {} - {}", code, msg) }
        }
    }
}
