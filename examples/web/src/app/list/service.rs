use super::storage_type::ListStorage;
use super::{List, ListId};
use crate::db::list::storage::PostgresListStorage;
use crate::db::persistence::{PersistencePool, PostgresPersistence};
use crate::error::ApiResult;

pub fn create_postgres_list_service(
    persistence: PostgresPersistence,
) -> ListService<PostgresPersistence> {
    ListService {
        persistence,
        list_storage: Box::new(PostgresListStorage {}),
    }
}

pub struct ListService<P>
where
    P: PersistencePool,
{
    persistence: P,
    list_storage: Box<dyn ListStorage<P::Conn>>,
}

impl<P> ListService<P>
where
    P: PersistencePool,
{
    pub async fn get_list_opt(&self, list_id: ListId) -> ApiResult<Option<List>> {
        let mut conn = self.persistence.get_connection().await?;
        let list = self.list_storage.get_list_opt(&mut conn, list_id).await?;
        Ok(list)
    }
}
