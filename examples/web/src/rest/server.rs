use super::server_utils;
use crate::config;
use crate::db::persistence::create_postgres_pool;
use crate::error::SyncStdError;
use crate::rest::context::{RestGlobalContext, RestReqContext};
use crate::rest::prelude::*;
use crate::rest::routes::{self, Resolver};
use crate::rest::types::REST_INTERNAL_SERVER_ERROR;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::sync::Arc;

/// Waits for the Ctrl+C signal for graceful shutdown backend
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

pub async fn start_server() -> StdResult<()> {
    let pool = create_postgres_pool().await;
    // let persistence = ood_persistence::bb8_postgres::new(&pool);
    let context = Arc::new(RestGlobalContext { pool });

    let new_service = make_service_fn(move |_| {
        let context = context.clone();
        async {
            Ok::<_, SyncStdError>(service_fn(move |req| process_request(req, context.clone())))
        }
    });

    let port = config::server::PORT();
    let addr = ([0, 0, 0, 0], port).into();
    let server = Server::bind(&addr)
        .serve(new_service)
        .with_graceful_shutdown(shutdown_signal());

    info!("🚀 Server listening on http://localhost:{}", port);

    server.await?;

    Ok(())
}

fn split_request_uri_path(uri_path: &str) -> Vec<&str> {
    uri_path
        .split('/')
        .filter(|part| !part.is_empty())
        .collect()
}

async fn process_request(
    req: Request<Body>,
    context: Arc<RestGlobalContext>,
) -> StdResult<Response<Body>> {
    let (req_parts, req_body) = req.into_parts();
    let query_params = server_utils::get_query_params(req_parts.uri.query());
    let req_variables = ReqVariables::new(req_body, query_params);

    let method = &req_parts.method;
    let uri_path_parts = &split_request_uri_path(req_parts.uri.path())[..];

    let req_context = RestReqContext {
        persistence: ood_persistence::bb8_postgres::new(&context.pool),
    };

    let route = routes::root::Router::from((method, uri_path_parts));
    let mut res = match route.resolve(req_context, req_variables).await {
        Err(_) => server_utils::create_json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            RestResponseData::<serde_json::Value>::error(REST_INTERNAL_SERVER_ERROR),
        )
        // TODO(pleshevskiy): investigate why `Send` is not implemented
        .unwrap(),
        Ok(res) => res,
    };

    if config::feature::CORS() {
        let headers = res.headers_mut();
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        );
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("HEAD, GET, POST, PUT, PATCH"),
        );
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("Authorization, Content-Type"),
        );
    }

    Ok(res)
}
