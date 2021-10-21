use crate::error;

/// The pool is one of the main features to be realized in persistence.
///
/// Any implementation (database, file storage, memory or others) must be able to get a connection
/// to manipulate it afterwards.
pub trait PersistencePool {
    /// `ConnectionClient` implementation that persistence can can return.
    type Conn: ConnectionClient;

    /// Returns new connection.
    ///
    /// # Errors
    ///
    /// Returns `PersistenceError` if pool cannot get a connection.
    fn get_connection(&self) -> crate::Result<Self::Conn>;
}

/// Connection client knows about the inner connection, and also knows how to create transactions.
pub trait ConnectionClient {
    /// Inner connection
    type InnerConn;

    /// `TransactionClient` implementation in which the connection can be updated.
    ///
    /// **Note:** requires nightly rust channel and enabling the `nightly` feature.
    #[cfg(feature = "nightly")]
    type Trx<'t>: TransactionClient;

    /// Returns inner connection
    fn inner(&mut self) -> &mut Self::InnerConn;

    /// Updates connection to transaction.
    ///
    /// **Note:** requires nightly rust channel and enabling the `nightly` feature.
    ///
    /// # Errors
    ///
    /// Returns `PersistenceError` if connection cannot update to transaction.
    #[cfg(feature = "nightly")]
    fn start_transaction(&mut self) -> crate::Result<Self::Trx<'_>>;
}

/// Transaction client is updated connection client that can additionally commit and rollback data
/// in transactions.
///
/// **Note:** requires nightly rust channel and enabling the `nightly` feature.
#[cfg(feature = "nightly")]
pub trait TransactionClient: ConnectionClient {
    /// Consumes the transaction, committing all changes made within it.
    ///
    /// # Errors
    ///
    /// Returns `PersistenceError` if transaction cannot commit
    fn commit(self) -> crate::Result<()>;

    /// Rolls the transaction back, discarding all changes made within it.
    ///
    /// # Errors
    ///
    /// Returns `PersistenceError` if transaction cannot rolls back.
    fn rollback(self) -> crate::Result<()>;
}
