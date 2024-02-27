pub mod crawler;
pub mod error;
pub mod nlpcut;
pub mod search;
pub type Result<T> = std::result::Result<T, error::AppError>;
