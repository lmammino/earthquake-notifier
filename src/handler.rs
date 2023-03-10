use crate::{config::Config, earthquake::Earthquake, fetcher::Fetcher, matcher::matching};
use std::error::Error;

pub async fn handler<F: Fetcher + Sync>(
    config: &Config,
    fetcher: &F,
) -> Result<String, Box<dyn Error>> {
    let recent_quakes = fetcher.fetch_last_hour().await?;
    let matches: Vec<&Earthquake> = matching(
        &recent_quakes,
        &config.center,
        config.max_distance_meters,
        config.min_magnitude,
    )
    .collect();

    println!("matches: {:#?}", matches);
    for quake in matches {
        println!("quake: {:#?}", quake);
    }
    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok("DONE".to_string())
}
