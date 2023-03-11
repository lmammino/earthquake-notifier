use geoutils::Location;
use std::{env, error::Error};

pub struct Config {
    pub max_distance_meters: f64,
    pub min_magnitude: f64,
    pub center: Location,
}

impl Config {
    pub fn new(max_distance_meters: f64, min_magnitude: f64, center: Location) -> Self {
        Self {
            max_distance_meters,
            min_magnitude,
            center,
        }
    }

    pub fn try_from_env() -> Result<Self, Box<dyn Error>> {
        let max_distance_meters = env::var("MAX_DISTANCE_METERS")?.parse()?;
        let min_magnitude = env::var("MIN_MAGNITUDE")?.parse()?;
        let lat_long = env::var("CENTER_LAT_LON")?;
        let (lat, lon) = lat_long.split_once(',').ok_or("missing comma")?;
        let lat: f64 = lat.parse()?;
        let lon: f64 = lon.parse()?;
        let center = Location::new(lat, lon);
        Ok(Self::new(max_distance_meters, min_magnitude, center))
    }
}
