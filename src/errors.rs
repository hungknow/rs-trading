
pub type Result<T> = std::result::Result<T, TaError>;

#[derive(Debug)]
pub enum TaError {
    InvalidParameter
}