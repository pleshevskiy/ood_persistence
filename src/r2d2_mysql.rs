#[cfg(feature = "nightly")]
use crate::syn::TransactionClient;
use crate::syn::{ConnectionClient, PersistencePool};
pub use r2d2::{Pool, PooledConnection};
pub use r2d2_mysql::mysql;
pub use r2d2_mysql::MysqlConnectionManager as Manager;

/// Inner connection of r2d2 implementation.
pub type InnerConn = PooledConnection<Manager>;
/// Inner transaction of `mysql`.
#[cfg(feature = "nightly")]
pub type InnerTrx<'t> = mysql::Transaction<'t>;

/// It creates new persistence of r2d2 mysql implementation.
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

/// Connection wrap over r2d2 mysql inner connection.
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
            .start_transaction(mysql::TxOpts::default())
            .map_err(|_| crate::Error::UpgradeToTransaction)
            .map(Transaction)
    }
}

/// Transaction wrap over `mysql` transaction.
///
/// **Note:** requires nightly rust channel and enabling the `nightly` feature.
///
/// # Limits
///
/// Mysql doesn't support nested transactions
#[cfg(feature = "nightly")]
pub struct Transaction<'me>(InnerTrx<'me>);

#[cfg(feature = "nightly")]
impl<'me> ConnectionClient for Transaction<'me> {
    type InnerConn = InnerTrx<'me>;

    #[cfg(feature = "nightly")]
    type Trx<'t> = Transaction<'t>;

    fn inner(&mut self) -> &mut Self::InnerConn {
        &mut self.0
    }

    #[cfg(feature = "nightly")]
    fn start_transaction(&mut self) -> crate::Result<Self::Trx<'_>> {
        // Mysql doesn't support nested transactions
        Err(crate::Error::UpgradeToNestedTransaction)
    }
}

#[cfg(feature = "nightly")]
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
