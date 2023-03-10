use crate::earthquake::Earthquake;
use async_trait::async_trait;
use aws_sdk_eventbridge::Client;
use std::error::Error;

#[async_trait]
pub trait Notifier {
    async fn notify(&self, earthquakes: &[Earthquake]) -> Result<(), Box<dyn Error>>;
}

pub struct EventBridgeNotifier {
    client: Client,
    event_bus: String,
}

impl EventBridgeNotifier {
    pub fn new(client: Client, event_bus: String) -> Self {
        Self { event_bus, client }
    }
}

#[async_trait]
impl Notifier for EventBridgeNotifier {
    async fn notify(&self, earthquakes: &[Earthquake]) -> Result<(), Box<dyn Error>> {
        // TODO: implement
        todo!()
    }
}
