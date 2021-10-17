use crate::config;
use ood_persistence::bb8_postgres::{tokio_postgres, NoTlsManager};
pub use ood_persistence::bb8_postgres::{
    NoTlsConnection as PostgresConnection, NoTlsPersistence as PostgresPersistence,
    NoTlsPool as PostgresPool, Transaction as PostgresTransaction,
};
pub use ood_persistence::{
    asyn::{ConnectionClient, PersistencePool, TransactionClient},
    error::Result as QueryResult,
};

pub async fn create_postgres_pool() -> PostgresPool {
    let db_conn_config = config::database::URL()
        .parse()
        .expect("Failed to convert database url to database config");

    let manager = NoTlsManager::new(db_conn_config, tokio_postgres::NoTls);
    let pool = PostgresPool::builder()
        .max_size(config::database::pool::MAX_SIZE())
        .build(manager)
        .await
        .expect("Failed to create database pool");

    debug!("Created database DatabaseConnection pool successfully");

    pool
}

pub fn try_get_one<Db, App>(row: tokio_postgres::Row) -> Result<App, tokio_postgres::Error>
where
    Db: postgres_types::FromSqlOwned,
    App: From<Db>,
{
    row.try_get(0).map(From::<Db>::from)
}
