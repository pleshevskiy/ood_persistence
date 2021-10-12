use crate::error;
use crate::syn::{ConnectionClient, PersistencePool};

pub use r2d2::{Pool, PooledConnection};
pub use r2d2_postgres::postgres;
pub use r2d2_postgres::PostgresConnectionManager as Manager;

pub type NoTlsManager = Manager<postgres::NoTls>;
pub type NoTlsPersistence<'p> = Persistence<'p, NoTlsManager>;
pub type NoTlsConnection<'p> = Connection<NoTlsManager>;
pub type NoTlsPool = Pool<NoTlsManager>;

pub type InnerConn<M> = PooledConnection<M>;

pub fn new<M>(pool: &Pool<M>) -> Persistence<M>
where
    M: r2d2::ManageConnection,
{
    Persistence(pool)
}

#[derive(Clone)]
pub struct Persistence<'p, M: r2d2::ManageConnection>(&'p Pool<M>);

#[async_trait]
impl<'p, M> PersistencePool for Persistence<'p, M>
where
    M: r2d2::ManageConnection + Send + Sync,
{
    type Conn = Connection<M>;

    fn get_connection(&self) -> error::Result<Self::Conn> {
        self.0
            .get()
            .map_err(|_| error::PersistenceError::GetConnection)
            .map(Connection)
    }
}

pub struct Connection<M: r2d2::ManageConnection>(InnerConn<M>);

impl<M> ConnectionClient for Connection<M>
where
    M: r2d2::ManageConnection,
{
    type InnerConn = InnerConn<M>;

    fn inner(&mut self) -> &mut Self::InnerConn {
        &mut self.0
    }
}
