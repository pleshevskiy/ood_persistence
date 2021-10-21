use std::error;
use std::fmt;

pub type SyncStdError = Box<dyn error::Error + Send + Sync + 'static>;
pub type StdResult<T> = Result<T, SyncStdError>;
pub type ApiResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    PersistenceError(ood_persistence::Error),
    Rest(RestKind),
    Serde(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PersistenceError(err) => write!(f, "{}", err),
            Self::Rest(err) => write!(f, "{}", err),
            Self::Serde(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<ood_persistence::Error> for Error {
    fn from(err: ood_persistence::Error) -> Self {
        Self::PersistenceError(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Self::Rest(RestKind::Hyper(err))
    }
}

impl From<hyper::http::Error> for Error {
    fn from(err: hyper::http::Error) -> Self {
        Self::Rest(RestKind::HyperHttp(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}

#[derive(Debug)]
pub enum RestKind {
    Hyper(hyper::Error),
    HyperHttp(hyper::http::Error),
}

impl fmt::Display for RestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hyper(err) => write!(f, "{}", err),
            Self::HyperHttp(err) => write!(f, "{}", err),
        }
    }
}
