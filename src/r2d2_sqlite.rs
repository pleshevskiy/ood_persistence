#[cfg(feature = "nightly")]
use crate::syn::TransactionClient;
use crate::syn::{ConnectionClient, PersistencePool};
pub use r2d2::{Pool, PooledConnection};
pub use r2d2_sqlite::rusqlite;
pub use r2d2_sqlite::SqliteConnectionManager as Manager;

/// Inner connection of r2d2 implementation.
pub type InnerConn = PooledConnection<Manager>;
/// Inner transaction of rusqlite.
pub type InnerTrx<'t> = rusqlite::Transaction<'t>;

/// It creates new persistence of r2d2 sqlite implementation.
#[must_use]
pub fn new(pool: &Pool<Manager>) -> Persistence {
    Persistence(pool)
}

/// Persistence wrap over r2d2 pool.
#[derive(Clone)]
pub struct Persistence<'p>(&'p Pool<Manager>);

impl PersistencePool for Persistence<'_> {
    type Conn = Connection;

    fn get_connection(&self) -> crate::Result<Self::Conn> {
        self.0
            .get()
            .map_err(|_| crate::Error::GetConnection)
            .map(Connection)
    }
}

/// Connection wrap over r2d2 sqlite inner connection.
pub struct Connection(InnerConn);

impl ConnectionClient for Connection {
    type InnerConn = InnerConn;

    #[cfg(feature = "nightly")]
    type Trx<'t> = Transaction<'t>;

    fn inner(&mut self) -> &mut Self::InnerConn {
        &mut self.0
    }

    #[cfg(feature = "nightly")]
    fn start_transaction(&mut self) -> crate::Result<Self::Trx<'_>> {
        self.0
            .transaction()
            .map_err(|_| crate::Error::UpgradeToTransaction)
            .map(Transaction)
    }
}

/// Transaction wrap over rusqlite transaction.
///
/// **Note:** requires nightly rust channel and enabling the `nightly` feature.
///
/// # Limits
///
/// It doesn't support nested transaction, because the transaction in `rusqlite`
/// requires `DerefMut`, which cannot be implemented at the moment. 😣
pub struct Transaction<'me>(InnerTrx<'me>);

impl<'me> ConnectionClient for Transaction<'me> {
    type InnerConn = InnerTrx<'me>;

    #[cfg(feature = "nightly")]
    type Trx<'t> = Transaction<'t>;

    fn inner(&mut self) -> &mut Self::InnerConn {
        &mut self.0
    }

    #[cfg(feature = "nightly")]
    fn start_transaction(&mut self) -> crate::Result<Self::Trx<'_>> {
        // At the moment we cannot implement nested transaction because
        // the transaction in `rusqlite` 😣 We need to implement
        // our box for transaction if we want to use it here.
        Err(crate::Error::UpgradeToNestedTransaction)
        // self.0
        //     .transaction()
        //     .map_err(|_| error::PersistenceError::UpgradeToTransaction)
        //     .map(Transaction)
    }
}

impl TransactionClient for Transaction<'_> {
    fn commit(self) -> crate::Result<()> {
        self.0.commit().map_err(|_| crate::Error::CommitTransaction)
    }
    fn rollback(self) -> crate::Result<()> {
        self.0
            .rollback()
            .map_err(|_| crate::Error::RollbackTransaction)
    }
}
