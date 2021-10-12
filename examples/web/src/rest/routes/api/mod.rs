use crate::rest::routes::*;

mod list;

pub enum Router {
    List(list::Router),
}

impl MaybeFrom<RouteParts<'_>> for Router {
    fn maybe_from((method, uri_path_parts): RouteParts<'_>) -> Option<Self> {
        let rest_parts = &uri_path_parts[1..];
        uri_path_parts.get(0).copied().and_then(|part| match part {
            "lists" => list::Router::maybe_from((method, rest_parts)).map(Self::List),
            _ => None,
        })
    }
}

#[async_trait]
impl Resolver for Router {
    async fn resolve(&self, ctx: RestReqContext<'_>, vars: ReqVariables<'_>) -> RestResult {
        let mut res = match self {
            Self::List(router) => router.resolve(ctx, vars).await?,
        };

        res.headers_mut().append(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        Ok(res)
    }
}
