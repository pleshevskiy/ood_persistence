use crate::app::list::controller::create_postgres_list_controller;
use crate::rest::routes::*;
use crate::rest::server_utils::{create_not_found_err_json_response, create_ok_json_response};

pub enum Router {
    GetListById(String),
}

impl MaybeFrom<RouteParts<'_>> for Router {
    fn maybe_from((method, uri_path_parts): RouteParts<'_>) -> Option<Self> {
        match (method, uri_path_parts) {
            (&Method::GET, [list_id]) => Some(Self::GetListById(list_id.to_string())),
            _ => None,
        }
    }
}

#[async_trait]
impl Resolver for Router {
    async fn resolve(&self, ctx: RestReqContext<'_>, vars: ReqVariables<'_>) -> RestResult {
        let controller = create_postgres_list_controller(ctx.persistence);
        match self {
            Self::GetListById(list_id) => {
                let res = controller.get_list_opt(list_id.parse().ok()).await?;
                match res {
                    Some(list) => create_ok_json_response(list),
                    None => create_not_found_err_json_response("List not found"),
                }
            }
        }
    }
}
