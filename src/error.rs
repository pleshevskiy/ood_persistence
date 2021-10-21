use std::fmt;

#[cfg(feature = "bb8_postgres")]
use bb8_postgres::tokio_postgres::Error as PostgresError;
#[cfg(all(feature = "r2d2_postgres", not(feature = "bb8_postgres")))]
use r2d2_postgres::postgres::Error as PostgresError;

#[cfg(feature = "r2d2_sqlite")]
use r2d2_sqlite::rusqlite::Error as RusqliteError;

/// A helper type for any result with persistence error.
///
/// Use this type in your repository or in something else that implements methods for your persistence.
pub type Result<T> = std::result::Result<T, Error>;

#[deprecated]
#[allow(clippy::module_name_repetitions)]
pub use Error as PersistenceError;

/// All supported kinds of persistence error
#[derive(Debug)]
pub enum Error {
    /// Returns if we cannot get a connection from pool.
    GetConnection,
    /// Returns if we cannot upgrade connection to transaction.
    #[cfg(feature = "nightly")]
    UpgradeToTransaction,
    /// Returns if we cannot commit transaction.
    #[cfg(feature = "nightly")]
    CommitTransaction,
    /// Returns if we cannot rolls back transaction.
    #[cfg(feature = "nightly")]
    RollbackTransaction,
    /// Rest persistence errors contains here.
    PersistenceError(Box<dyn std::error::Error>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::GetConnection => f.write_str("Cannot get connection"),
            #[cfg(feature = "nightly")]
            Error::UpgradeToTransaction => f.write_str("Cannot upgrade connection to transaction"),
            #[cfg(feature = "nightly")]
            Error::CommitTransaction => f.write_str("Cannot commit changes of transaction"),
            #[cfg(feature = "nightly")]
            Error::RollbackTransaction => f.write_str("Cannot rolls transaction back"),
            Error::PersistenceError(err) => write!(f, "DbError: {}", err),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(any(feature = "r2d2_postgres", feature = "bb8_postgres"))]
impl From<PostgresError> for Error {
    fn from(err: PostgresError) -> Self {
        Self::PersistenceError(Box::new(err))
    }
}

#[cfg(feature = "r2d2_sqlite")]
impl From<RusqliteError> for Error {
    fn from(err: RusqliteError) -> Self {
        Self::PersistenceError(Box::new(err))
    }
}
