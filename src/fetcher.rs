use crate::earthquake::Earthquake;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Url;
use std::error::Error;

const BASE_URL: &str = "https://webservices.ingv.it/fdsnws/event/1/query";

#[async_trait]
pub trait Fetcher {
    async fn fetch(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<Earthquake>, Box<dyn Error>>;

    async fn fetch_last_hour(&self) -> Result<Vec<Earthquake>, Box<dyn Error>> {
        let now = Utc::now();
        let one_hour_ago = now - chrono::Duration::hours(20);

        self.fetch(one_hour_ago, now).await
    }
}

pub struct IngvFetcher {}

impl IngvFetcher {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Fetcher for IngvFetcher {
    async fn fetch(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<Earthquake>, Box<dyn Error>> {
        let start_time_fmt = start_time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let end_time_fmt = end_time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let url = Url::parse_with_params(
            BASE_URL,
            &[
                (
                    "starttime",
                    start_time_fmt[..start_time_fmt.len() - 1].to_string(), // NOTE: crappy hack to remove the Z
                ),
                (
                    "endtime",
                    end_time_fmt[..end_time_fmt.len() - 1].to_string(),
                ),
                ("format", "text".to_string()),
            ],
        )?;

        let resp = reqwest::get(url).await?.text().await?;

        let result: Vec<Earthquake> = resp
            .lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_last_hour() {
        let fetcher = IngvFetcher {};
        let result = fetcher.fetch_last_hour().await.unwrap();
        println!("result: {:#?}", result);
    }
}
