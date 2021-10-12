use web_example::config::load_env_config;
use web_example::error::StdResult;
use web_example::rest::server::start_server;

#[tokio::main]
async fn main() -> StdResult<()> {
    load_env_config();
    start_server().await?;
    Ok(())
}
