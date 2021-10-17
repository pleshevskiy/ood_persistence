use super::{List, ListId};
use crate::db::persistence::{ConnectionClient, QueryResult};

#[async_trait]
pub trait ListStorage<Conn>: Send + Sync
where
    Conn: ConnectionClient,
{
    async fn get_list_opt(&self, conn: &mut Conn, id: ListId) -> QueryResult<Option<List>>;

    async fn add_list(&self, conn: &mut Conn::Trx<'_>, name: &str) -> QueryResult<List>;
}
