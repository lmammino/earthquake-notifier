/*
#EventID|Time                       |Latitude|Longitude|Depth/Km|Author|Catalog|Contributor|ContributorID|MagType|Magnitude|MagAuthor|EventLocationName|EventType
34317601|2023-03-10T16:13:34.380000|43.2798|12.3778|7.3|SURVEY-INGV||||ML|1.3|--|5 km SE Umbertide (PG)|earthquake
34316691|2023-03-10T15:36:40.330000|43.2095|11.034|7.3|SURVEY-INGV||||ML|1.9|--|6 km S Radicondoli (SI)|earthquake
34313851|2023-03-10T13:47:02.660000|43.2822|12.3948|8.7|SURVEY-INGV||||ML|1.2|--|6 km E Umbertide (PG)|earthquake
 */
use chrono::{DateTime, Utc};
use geoutils::Location;
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Earthquake {
    pub event_id: String,
    pub time: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub depth: f64,
    pub author: String,
    pub catalog: String,
    pub contributor: String,
    pub contributor_id: String,
    pub mag_type: String,
    pub magnitude: f64,
    pub mag_author: String,
    pub event_location_name: String,
    pub event_type: String,
}

impl FromStr for Earthquake {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('|');
        let event_id = parts.next().ok_or("missing event_id")?.to_string();
        let mut time = parts.next().ok_or("missing time")?.to_string();
        time.push('Z');
        let time = DateTime::parse_from_rfc3339(time.as_str()).map_err(|e| e.to_string())?;
        // let time = time.parse::<DateTime<Utc>>().map_err(|e| e.to_string())?;
        let latitude = parts
            .next()
            .ok_or("missing latitude")?
            .parse::<f64>()
            .map_err(|e| e.to_string())?;
        let longitude = parts
            .next()
            .ok_or("missing longitude")?
            .parse::<f64>()
            .map_err(|e| e.to_string())?;
        let depth = parts
            .next()
            .ok_or("missing depth")?
            .parse::<f64>()
            .map_err(|e| e.to_string())?;
        let author = parts.next().ok_or("missing author")?.to_string();
        let catalog = parts.next().ok_or("missing catalog")?.to_string();
        let contributor = parts.next().ok_or("missing contributor")?.to_string();
        let contributor_id = parts.next().ok_or("missing contributor_id")?.to_string();
        let mag_type = parts.next().ok_or("missing mag_type")?.to_string();
        let magnitude = parts
            .next()
            .ok_or("missing magnitude")?
            .parse::<f64>()
            .map_err(|e| e.to_string())?;
        let mag_author = parts.next().ok_or("missing mag_author")?.to_string();
        let event_location_name = parts
            .next()
            .ok_or("missing event_location_name")?
            .to_string();
        let event_type = parts.next().ok_or("missing event_type")?.to_string();
        Ok(Earthquake {
            event_id,
            time: time.into(),
            latitude,
            longitude,
            depth,
            author,
            catalog,
            contributor,
            contributor_id,
            mag_type,
            magnitude,
            mag_author,
            event_location_name,
            event_type,
        })
    }
}

impl Earthquake {
    pub fn center(&self) -> Location {
        Location::new(self.latitude, self.longitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "34317601|2023-03-10T16:13:34.380000|43.2798|12.3778|7.3|SURVEY-INGV||||ML|1.3|--|5 km SE Umbertide (PG)|earthquake";
        let earthquake = input.parse::<Earthquake>().expect("failed to parse");
        assert_eq!(
            earthquake,
            Earthquake {
                event_id: "34317601".to_string(),
                time: "2023-03-10T16:13:34.380Z".parse::<DateTime<Utc>>().unwrap(),
                latitude: 43.2798,
                longitude: 12.3778,
                depth: 7.3,
                author: "SURVEY-INGV".to_string(),
                catalog: "".to_string(),
                contributor: "".to_string(),
                contributor_id: "".to_string(),
                mag_type: "ML".to_string(),
                magnitude: 1.3,
                mag_author: "--".to_string(),
                event_location_name: "5 km SE Umbertide (PG)".to_string(),
                event_type: "earthquake".to_string(),
            }
        );
    }
}
