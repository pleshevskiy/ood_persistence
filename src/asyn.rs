use crate::error;

#[async_trait]
pub trait PersistencePool: Send + Sync {
    type Conn: ConnectionClient;

    async fn get_connection(&self) -> error::Result<Self::Conn>;
}

pub trait ConnectionClient {
    type InnerConn;

    fn inner(&mut self) -> &mut Self::InnerConn;
}
