use aws_sdk_eventbridge::Client;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use notifier::EventBridgeNotifier;
use serde_json::Value;
use std::env;

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

    let event_bus = env::var("EVENT_BUS").ok();
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let config = Config::try_from_env().map_err(|e| Error::from(e.to_string()))?;
    let fetcher = IngvFetcher::new();
    let notifier = EventBridgeNotifier::new(client, event_bus);
    let handler =
        service_fn(|_event: LambdaEvent<Value>| handler::handler(&config, &fetcher, &notifier));
    lambda_runtime::run(handler).await?;
    Ok(())
}
