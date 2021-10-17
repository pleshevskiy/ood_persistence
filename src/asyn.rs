use crate::error;

#[async_trait]
pub trait PersistencePool: Send + Sync {
    type Conn: ConnectionClient;

    async fn get_connection(&self) -> error::Result<Self::Conn>;
}

#[cfg_attr(feature = "nightly", async_trait)]
pub trait ConnectionClient {
    type InnerConn;

    #[cfg(feature = "nightly")]
    type Trx<'t>: TransactionClient;

    fn inner(&mut self) -> &mut Self::InnerConn;

    #[cfg(feature = "nightly")]
    async fn start_transaction(&mut self) -> error::Result<Self::Trx<'_>>;
}

#[cfg(feature = "nightly")]
#[async_trait]
pub trait TransactionClient: ConnectionClient {
    async fn commit(self) -> error::Result<()>;

    async fn rollback(self) -> error::Result<()>;
}
