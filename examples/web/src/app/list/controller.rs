use super::service::{create_postgres_list_service, ListService};
use super::{List, ListId};
use crate::db::persistence::PersistencePool;
use crate::db::persistence::PostgresPersistence;
use crate::error::ApiResult;

pub fn create_postgres_list_controller(
    persistence: PostgresPersistence,
) -> ListController<PostgresPersistence> {
    ListController {
        list_service: create_postgres_list_service(persistence),
    }
}

pub struct ListController<P>
where
    P: PersistencePool,
{
    list_service: ListService<P>,
}

impl<P> ListController<P>
where
    P: PersistencePool,
{
    pub async fn get_list_opt(&self, list_id: Option<ListId>) -> ApiResult<Option<List>> {
        match list_id {
            Some(list_id) => self.list_service.get_list_opt(list_id).await,
            _ => Ok(None),
        }
    }
}
