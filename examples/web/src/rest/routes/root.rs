use crate::config;
use crate::rest::routes::*;

#[non_exhaustive]
pub enum Router {
    #[allow(clippy::upper_case_acronyms)]
    CORS,
    HealthCheck,
    NotFound,
    Api(api::Router),
}

impl From<RouteParts<'_>> for Router {
    fn from((method, uri_path_parts): RouteParts<'_>) -> Self {
        match (method, uri_path_parts) {
            (&Method::OPTIONS, _) if config::feature::CORS() => Self::CORS,
            (_, &["api", ..]) => api::Router::maybe_from((method, &uri_path_parts[1..]))
                .map(Self::Api)
                .unwrap_or(Self::NotFound),
            (&Method::GET, &["health"]) => Self::HealthCheck,
            _ => Self::NotFound,
        }
    }
}

#[async_trait]
impl Resolver for Router {
    async fn resolve(&self, ctx: RestReqContext<'_>, vars: ReqVariables<'_>) -> RestResult {
        let res = match self {
            Self::CORS => Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())?,
            Self::Api(route) => route.resolve(ctx, vars).await?,
            Self::HealthCheck => Response::builder()
                .status(StatusCode::OK)
                .body(Body::from("Ok"))?,
            Self::NotFound => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))?,
        };

        Ok(res)
    }
}
