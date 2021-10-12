use crate::error;

pub trait PersistencePool {
    type Conn: ConnectionClient;

    fn get_connection(&self) -> error::Result<Self::Conn>;
}

pub trait ConnectionClient {
    type InnerConn;

    fn inner(&mut self) -> &mut Self::InnerConn;
}
