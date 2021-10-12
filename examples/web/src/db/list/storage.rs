use super::DbList;
use crate::app::list::storage_type::ListStorage;
use crate::app::list::{List, ListId};
use crate::db::persistence::{try_get_one, ConnectionClient, PostgresConnection, QueryResult};
use postgres_types::Type;

pub struct PostgresListStorage {}

#[async_trait]
impl<'c> ListStorage<PostgresConnection<'c>> for PostgresListStorage {
    async fn get_list_opt(
        &self,
        conn: &mut PostgresConnection<'c>,
        list_id: ListId,
    ) -> QueryResult<Option<List>> {
        let inner_conn = conn.inner();

        let stmt = inner_conn
            .prepare_typed("select l from lists as l where l.id = $1", &[Type::INT4])
            .await?;

        inner_conn
            .query_opt(&stmt, &[&list_id])
            .await?
            .map(try_get_one::<DbList, _>)
            .transpose()
            .map_err(From::from)
    }
}
