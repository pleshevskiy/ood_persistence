use crate::rest::context::RestReqContext;
use crate::rest::prelude::*;

mod api;
pub mod root;

#[async_trait]
pub trait Resolver {
    async fn resolve(&self, ctx: RestReqContext<'_>, vars: ReqVariables<'_>) -> RestResult;
}

type RouteParts<'a> = (&'a Method, &'a [&'a str]);

trait MaybeFrom<T>: Sized {
    fn maybe_from(_: T) -> Option<Self>;
}
