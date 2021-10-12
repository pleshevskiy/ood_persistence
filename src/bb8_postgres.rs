use crate::asyn::{ConnectionClient, PersistencePool};
use crate::error;

pub use bb8::{Pool, PooledConnection};
pub use bb8_postgres::tokio_postgres;
pub use bb8_postgres::PostgresConnectionManager as Manager;

pub type NoTlsManager = Manager<tokio_postgres::NoTls>;
pub type NoTlsPersistence<'p> = Persistence<'p, NoTlsManager>;
pub type NoTlsConnection<'p> = Connection<'p, NoTlsManager>;
pub type NoTlsPool = Pool<NoTlsManager>;

pub type InnerConn<'p, M> = PooledConnection<'p, M>;

pub fn new<M>(pool: &Pool<M>) -> Persistence<M>
where
    M: bb8::ManageConnection,
{
    Persistence(pool)
}

#[derive(Clone)]
pub struct Persistence<'p, M: bb8::ManageConnection>(&'p Pool<M>);

#[async_trait]
impl<'p, M> PersistencePool for Persistence<'p, M>
where
    M: bb8::ManageConnection + Send + Sync,
{
    type Conn = Connection<'p, M>;

    async fn get_connection(&self) -> error::Result<Self::Conn> {
        self.0
            .get()
            .await
            .map_err(|_| error::PersistenceError::GetConnection)
            .map(Connection)
    }
}

pub struct Connection<'p, M: bb8::ManageConnection>(InnerConn<'p, M>);

impl<'c, M> ConnectionClient for Connection<'c, M>
where
    M: bb8::ManageConnection,
{
    type InnerConn = InnerConn<'c, M>;

    fn inner(&mut self) -> &mut Self::InnerConn {
        &mut self.0
    }
}
