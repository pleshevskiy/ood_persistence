use crate::error;
#[cfg(feature = "nightly")]
use crate::syn::TransactionClient;
use crate::syn::{ConnectionClient, PersistencePool};

pub use r2d2::{Pool, PooledConnection};
pub use r2d2_postgres::postgres;
pub use r2d2_postgres::PostgresConnectionManager as Manager;

pub type InnerConn<M> = PooledConnection<M>;
pub type InnerTrx<'t> = postgres::Transaction<'t>;

pub type NoTlsManager = Manager<postgres::NoTls>;
pub type NoTlsPersistence<'p> = Persistence<'p, NoTlsManager>;
pub type NoTlsConnection = Connection<NoTlsManager>;
pub type NoTlsInnerConn = InnerConn<NoTlsManager>;
pub type NoTlsPool = Pool<NoTlsManager>;

pub fn new<M>(pool: &Pool<M>) -> Persistence<M>
where
    M: r2d2::ManageConnection,
{
    Persistence(pool)
}

#[derive(Clone)]
pub struct Persistence<'p, M>(&'p Pool<M>)
where
    M: r2d2::ManageConnection;

impl<'p> PersistencePool for NoTlsPersistence<'p> {
    type Conn = NoTlsConnection;

    fn get_connection(&self) -> error::Result<Self::Conn> {
        self.0
            .get()
            .map_err(|_| error::PersistenceError::GetConnection)
            .map(Connection)
    }
}

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
    fn start_transaction(&mut self) -> error::Result<Self::Trx<'_>> {
        self.0
            .transaction()
            .map_err(|_| error::PersistenceError::UpgradeToTransaction)
            .map(Transaction)
    }
}

#[cfg(feature = "nightly")]
pub struct Transaction<'me>(InnerTrx<'me>);

#[cfg(feature = "nightly")]
impl<'me> ConnectionClient for Transaction<'me> {
    type InnerConn = InnerTrx<'me>;

    type Trx<'t> = Transaction<'t>;

    fn inner(&mut self) -> &mut Self::InnerConn {
        &mut self.0
    }

    fn start_transaction(&mut self) -> error::Result<Self::Trx<'_>> {
        self.0
            .transaction()
            .map_err(|_| error::PersistenceError::UpgradeToTransaction)
            .map(Transaction)
    }
}

#[cfg(feature = "nightly")]
impl TransactionClient for Transaction<'_> {
    fn commit(self) -> error::Result<()> {
        self.0
            .commit()
            .map_err(|_| error::PersistenceError::CommitTransaction)
    }

    fn rollback(self) -> error::Result<()> {
        self.0
            .rollback()
            .map_err(|_| error::PersistenceError::RollbackTransaction)
    }
}
