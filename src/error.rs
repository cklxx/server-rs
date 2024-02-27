use axum::response::IntoResponse;
use deadpool_diesel::{InteractError, PoolError};
use tantivy::{query::QueryParserError, TantivyError};
#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error>>,
    pub types: AppErrorType,
}

impl AppError {
    fn new(
        message: Option<String>,
        cause: Option<Box<dyn std::error::Error>>,
        types: AppErrorType,
    ) -> Self {
        Self {
            message,
            cause,
            types,
        }
    }
    fn from_err(cause: Box<dyn std::error::Error>, types: AppErrorType) -> Self {
        Self::new(None, Some(cause), types)
    }
    // fn from_str(msg: &str, types: AppErrorType) -> Self {
    //     Self::new(Some(msg.to_string()), None, types)
    // }
    pub fn notfound_opt(message: Option<String>) -> Self {
        Self::new(message, None, AppErrorType::Notfound)
    }
    pub fn notfound_msg(msg: &str) -> Self {
        Self::notfound_opt(Some(msg.to_string()))
    }
    pub fn notfound() -> Self {
        Self::notfound_msg("没有找到符合条件的数据")
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError {}

impl From<TantivyError> for AppError {
    fn from(err: TantivyError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Engine)
    }
}

impl From<QueryParserError> for AppError {
    fn from(err: QueryParserError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Engine)
    }
}

impl From<PoolError> for AppError {
    fn from(err: PoolError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Db)
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Db)
    }
}

impl From<InteractError> for AppError {
    fn from(err: InteractError) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Db)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let msg = match self.message {
            Some(msg) => msg.clone(),
            None => "有错误发生".to_string(),
        };
        msg.into_response()
    }
}

#[derive(Debug)]
pub enum AppErrorType {
    Db,
    Engine,
    Template,
    Notfound,
}
