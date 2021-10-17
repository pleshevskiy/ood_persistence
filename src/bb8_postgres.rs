#[cfg(feature = "nightly")]
use crate::asyn::TransactionClient;
use crate::asyn::{ConnectionClient, PersistencePool};
use crate::error;

pub use bb8::{Pool, PooledConnection};
pub use bb8_postgres::tokio_postgres;
pub use bb8_postgres::PostgresConnectionManager as Manager;

/// Inner connection of bb8 implementation.
pub type InnerConn<'p, M> = PooledConnection<'p, M>;
/// Inner connection of tokio postgres connection.
pub type InnerTrx<'p> = tokio_postgres::Transaction<'p>;

/// Alias for bb8 postgres no tls manager.
pub type NoTlsManager = Manager<tokio_postgres::NoTls>;
/// Alias for bb8 postgres no tls persistence.
pub type NoTlsPersistence<'p> = Persistence<'p, NoTlsManager>;
/// Alias for bb8 postgres no tls connection.
pub type NoTlsConnection<'p> = Connection<'p, NoTlsManager>;
/// Alias for bb8 postgres no tls inner connection.
pub type NoTlsInnerConn<'p> = InnerConn<'p, NoTlsManager>;
/// Alias for bb8 postgres no tls pool.
pub type NoTlsPool = Pool<NoTlsManager>;

/// It creates new persistence of bb8 postgres implementation.
#[must_use]
pub fn new<M>(pool: &Pool<M>) -> Persistence<M>
where
    M: bb8::ManageConnection,
{
    Persistence(pool)
}

/// Persistence wrap over bb8 pool.
#[derive(Clone)]
pub struct Persistence<'p, M>(&'p Pool<M>)
where
    M: bb8::ManageConnection;

#[async_trait]
impl<'p> PersistencePool for NoTlsPersistence<'p> {
    type Conn = NoTlsConnection<'p>;

    async fn get_connection(&self) -> error::Result<Self::Conn> {
        self.0
            .get()
            .await
            .map_err(|_| error::PersistenceError::GetConnection)
            .map(Connection)
    }
}

/// Connection wrap over bb8 postgres inner connection.
pub struct Connection<'p, M>(InnerConn<'p, M>)
where
    M: bb8::ManageConnection;

#[cfg_attr(feature = "nightly", async_trait)]
impl<'me> ConnectionClient for NoTlsConnection<'me> {
    type InnerConn = NoTlsInnerConn<'me>;

    #[cfg(feature = "nightly")]
    type Trx<'t> = Transaction<'t>;

    fn inner(&mut self) -> &mut Self::InnerConn {
        &mut self.0
    }

    #[cfg(feature = "nightly")]
    async fn start_transaction(&mut self) -> error::Result<Self::Trx<'_>> {
        self.0
            .transaction()
            .await
            .map_err(|_| error::PersistenceError::UpgradeToTransaction)
            .map(Transaction)
    }
}

/// Transaction wrap over tokio_postgres transaction.
///
/// **Note:** requires nightly rust channel and enabling the `nightly` feature.
#[cfg(feature = "nightly")]
pub struct Transaction<'p>(InnerTrx<'p>);

#[cfg(feature = "nightly")]
#[async_trait]
impl<'me> ConnectionClient for Transaction<'me> {
    type InnerConn = InnerTrx<'me>;

    type Trx<'t> = Transaction<'t>;

    fn inner(&mut self) -> &mut Self::InnerConn {
        &mut self.0
    }

    async fn start_transaction(&mut self) -> error::Result<Self::Trx<'_>> {
        self.0
            .transaction()
            .await
            .map_err(|_| error::PersistenceError::UpgradeToTransaction)
            .map(Transaction)
    }
}

#[cfg(feature = "nightly")]
#[async_trait]
impl<'me> TransactionClient for Transaction<'me> {
    async fn commit(self) -> error::Result<()> {
        self.0
            .commit()
            .await
            .map_err(|_| error::PersistenceError::CommitTransaction)
    }

    async fn rollback(self) -> error::Result<()> {
        self.0
            .rollback()
            .await
            .map_err(|_| error::PersistenceError::RollbackTransaction)
    }
}
