use crate::earthquake::Earthquake;
use async_trait::async_trait;
use aws_sdk_eventbridge::{model::PutEventsRequestEntry, Client};
use futures::future::join_all;
use std::error::Error;

#[async_trait]
pub trait Notifier {
    async fn notify(&self, earthquakes: &[&Earthquake]) -> Result<(), Box<dyn Error>>;
}

pub struct EventBridgeNotifier {
    client: Client,
    event_bus: Option<String>,
}

impl EventBridgeNotifier {
    pub fn new(client: Client, event_bus: Option<String>) -> Self {
        Self { event_bus, client }
    }
}

#[async_trait]
impl Notifier for EventBridgeNotifier {
    async fn notify(&self, earthquakes: &[&Earthquake]) -> Result<(), Box<dyn Error>> {
        let tasks = earthquakes
            .iter()
            .map(|earthquake| {
                let mut builder = PutEventsRequestEntry::builder()
                    .detail(serde_json::json!(earthquake).to_string())
                    .detail_type("earthquake".to_string())
                    .source("earthquake-notifier".to_string());

                if let Some(event_bus) = &self.event_bus {
                    builder = builder.event_bus_name(event_bus.to_string());
                }

                builder.build()
            })
            .map(|event| self.client.put_events().entries(event).send());

        // TODO: check if tasks failed and needs retry
        let _result = join_all(tasks).await;
        Ok(())
    }
}
