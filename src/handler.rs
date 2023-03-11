use crate::{
    config::Config, earthquake::Earthquake, fetcher::Fetcher, matcher::matching, notifier::Notifier,
};
use std::error::Error;

pub async fn handler<F: Fetcher + Sync, N: Notifier + Sync>(
    config: &Config,
    fetcher: &F,
    notifier: &N,
) -> Result<String, Box<dyn Error>> {
    let recent_quakes = fetcher.fetch_last_hour().await?;
    let matched_earthquakes: Vec<&Earthquake> = matching(
        &recent_quakes,
        &config.center,
        config.max_distance_meters,
        config.min_magnitude,
    )
    .collect();

    let found = matched_earthquakes.len();
    if found > 0 {
        notifier.notify(&matched_earthquakes).await?;
    }

    Ok(format!("Notified {found} earthquakes"))
}
