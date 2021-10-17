use crate::error;

pub trait PersistencePool {
    type Conn: ConnectionClient;

    fn get_connection(&self) -> error::Result<Self::Conn>;
}

pub trait ConnectionClient {
    type InnerConn;

    #[cfg(feature = "nightly")]
    type Trx<'t>: TransactionClient;

    fn inner(&mut self) -> &mut Self::InnerConn;

    #[cfg(feature = "nightly")]
    fn start_transaction(&mut self) -> error::Result<Self::Trx<'_>>;
}

#[cfg(feature = "nightly")]
pub trait TransactionClient: ConnectionClient {
    fn commit(self) -> error::Result<()>;

    fn rollback(self) -> error::Result<()>;
}
