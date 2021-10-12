use crate::db::persistence::{PostgresPersistence, PostgresPool};

pub struct RestGlobalContext {
    pub pool: PostgresPool,
}

#[derive(Clone)]
pub struct RestReqContext<'p> {
    pub persistence: PostgresPersistence<'p>,
}
