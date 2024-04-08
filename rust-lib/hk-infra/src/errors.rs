pub type Result<T> = std::result::Result<T, HkError>;

#[derive(Debug)]
pub enum HkError {
    UnknownError(String),
    InvalidParameter,
    CsvError(String),
    CsvMissingColumn(String),
    HkDataError(String),
    HkServerError(i32, String),
    HkFFIUnimplemented(i32),
}

// impl From<csv::Error> for HkError {
//     fn from(e: csv::Error) -> Self {
//         HkError::CsvError(e)
//     }
// }

impl std::fmt::Display for HkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HkError::UnknownError(msg) => write!(f, "{}", msg),
            HkError::InvalidParameter => write!(f, "Invalid Param"),
            HkError::CsvError(e) => { write!(f, "{}", e) }
            HkError::CsvMissingColumn(e) => { write!(f, "{}", e) }
            HkError::HkDataError(e) => { write!(f, "{}", e) }
            HkError::HkServerError(code, msg) => { write!(f, "Server Error: {} - {}", code, msg) }
            HkError::HkFFIUnimplemented(event) => { write!(f, "FFI unimplemented {}", event) }
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
