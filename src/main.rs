use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;

use crate::{config::Config, fetcher::IngvFetcher};
mod config;
mod earthquake;
mod fetcher;
mod handler;
mod matcher;
mod notifier;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    println!("Starting earthquake-monitor... ");

    let config = Config::try_from_env().map_err(|e| Error::from(e.to_string()))?;
    let fetcher = IngvFetcher::new();
    let handler = service_fn(|_event: LambdaEvent<Value>| handler::handler(&config, &fetcher));
    lambda_runtime::run(handler).await?;
    Ok(())
}
