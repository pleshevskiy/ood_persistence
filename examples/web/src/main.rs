use web_example::config::load_env_config;
use web_example::db::persistence::create_postgres_pool;
use web_example::error::StdResult;
use web_example::rest::server::start_server;

#[tokio::main]
async fn main() -> StdResult<()> {
    load_env_config();

    let pool = create_postgres_pool().await;
    start_server(pool).await?;
    Ok(())
}
