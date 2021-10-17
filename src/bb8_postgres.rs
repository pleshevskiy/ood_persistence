#[cfg(feature = "nightly")]
use crate::asyn::TransactionClient;
use crate::asyn::{ConnectionClient, PersistencePool};
use crate::error;

pub use bb8::{Pool, PooledConnection};
pub use bb8_postgres::tokio_postgres;
pub use bb8_postgres::PostgresConnectionManager as Manager;

pub type InnerConn<'p, M> = PooledConnection<'p, M>;
pub type InnerTrx<'p> = tokio_postgres::Transaction<'p>;

pub type NoTlsManager = Manager<tokio_postgres::NoTls>;
pub type NoTlsPersistence<'p> = Persistence<'p, NoTlsManager>;
pub type NoTlsConnection<'p> = Connection<'p, NoTlsManager>;
pub type NoTlsInnerConn<'p> = InnerConn<'p, NoTlsManager>;
pub type NoTlsPool = Pool<NoTlsManager>;

#[must_use]
pub fn new<M>(pool: &Pool<M>) -> Persistence<M>
where
    M: bb8::ManageConnection,
{
    Persistence(pool)
}

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
