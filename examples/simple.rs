use ood_persistence::r2d2_postgres::{postgres, NoTlsManager};
pub use ood_persistence::r2d2_postgres::{
    NoTlsConnection as PostgresConnection, NoTlsPersistence as PostgresPersistence,
    NoTlsPool as PostgresPool,
};
pub use ood_persistence::{
    error::Result as QueryResult,
    syn::{ConnectionClient, PersistencePool},
};

const DATABASE_URL: &str = "postgres://postgres:test@localhost:5577/x";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create pool to our database
    let pool = create_postgres_pool();
    // initialize persistence
    let persistence = ood_persistence::r2d2_postgres::new(&pool);
    // get connection to database
    let mut conn = persistence.get_connection()?;
    // we can query something
    let res: i32 = conn.inner().query_one("select 1", &[])?.get(0);
    assert_eq!(res, 1);

    Ok(())
}

pub fn create_postgres_pool() -> PostgresPool {
    let db_conn_config = DATABASE_URL
        .parse()
        .expect("Failed to convert database url to database config");

    let manager = NoTlsManager::new(db_conn_config, postgres::NoTls);
    PostgresPool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}
