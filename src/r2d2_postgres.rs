#[cfg(feature = "nightly")]
use crate::syn::TransactionClient;
use crate::syn::{ConnectionClient, PersistencePool};

pub use r2d2::{Pool, PooledConnection};
pub use r2d2_postgres::postgres;
pub use r2d2_postgres::PostgresConnectionManager as Manager;

/// Inner connection of r2d2 implementation.
pub type InnerConn<M> = PooledConnection<M>;
/// Inner transaction of postgres.
pub type InnerTrx<'t> = postgres::Transaction<'t>;

/// Alias for r2d2 postgres no tls manager.
pub type NoTlsManager = Manager<postgres::NoTls>;
/// Alias for r2d2 postgres no tls persistence.
pub type NoTlsPersistence<'p> = Persistence<'p, NoTlsManager>;
/// Alias for r2d2 postgres no tls connection.
pub type NoTlsConnection = Connection<NoTlsManager>;
/// Alias for r2d2 postgres no tls inner connection.
pub type NoTlsInnerConn = InnerConn<NoTlsManager>;
/// Alias for r2d2 postgres no tls pool.
pub type NoTlsPool = Pool<NoTlsManager>;

/// It creates new persistence of r2d2 postgres implementation.
#[must_use]
pub fn new<M>(pool: &Pool<M>) -> Persistence<M>
where
    M: r2d2::ManageConnection,
{
    Persistence(pool)
}

/// Persistence wrap over r2d2 pool.
#[derive(Clone)]
pub struct Persistence<'p, M>(&'p Pool<M>)
where
    M: r2d2::ManageConnection;

impl PersistencePool for NoTlsPersistence<'_> {
    type Conn = NoTlsConnection;

    fn get_connection(&self) -> crate::Result<Self::Conn> {
        self.0
            .get()
            .map_err(|_| crate::Error::GetConnection)
            .map(Connection)
    }
}

/// Connection wrap over r2d2 postgres inner connection.
pub struct Connection<M>(InnerConn<M>)
where
    M: r2d2::ManageConnection;

impl ConnectionClient for NoTlsConnection {
    type InnerConn = NoTlsInnerConn;

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

/// Transaction wrap over postgres transaction.
///
/// **Note:** requires nightly rust channel and enabling the `nightly` feature.
#[cfg(feature = "nightly")]
pub struct Transaction<'me>(InnerTrx<'me>);

#[cfg(feature = "nightly")]
impl<'me> ConnectionClient for Transaction<'me> {
    type InnerConn = InnerTrx<'me>;

    type Trx<'t> = Transaction<'t>;

    fn inner(&mut self) -> &mut Self::InnerConn {
        &mut self.0
    }

    fn start_transaction(&mut self) -> crate::Result<Self::Trx<'_>> {
        self.0
            .transaction()
            .map_err(|_| crate::Error::UpgradeToTransaction)
            .map(Transaction)
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
