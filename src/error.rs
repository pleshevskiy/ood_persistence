use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, PersistenceError>;

#[derive(Debug)]
pub enum PersistenceError {
    GetConnection,
    UpgradeToTransaction,
    CommitTransaction,
    RollbackTransaction,
    DbError(Box<dyn std::error::Error>),
}

impl fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PersistenceError::GetConnection => f.write_str("Cannot get connection"),
            PersistenceError::UpgradeToTransaction => {
                f.write_str("Cannot upgrade connection to transaction")
            }
            PersistenceError::CommitTransaction => f.write_str("Cannot commit transaction"),
            PersistenceError::RollbackTransaction => f.write_str("Cannot rollback transaction"),
            PersistenceError::DbError(err) => write!(f, "DbError: {}", err),
        }
    }
}

impl error::Error for PersistenceError {}

#[cfg(feature = "bb8_postgres")]
impl From<bb8_postgres::tokio_postgres::Error> for PersistenceError {
    fn from(err: bb8_postgres::tokio_postgres::Error) -> Self {
        Self::DbError(Box::new(err))
    }
}
