use crate::earthquake::Earthquake;
use geoutils::Location;

pub fn matching<'a>(
    earthquakes: &'a [Earthquake],
    center: &'a Location,
    max_distance_meters: f64,
    min_magnitude: f64,
) -> impl Iterator<Item = &'a Earthquake> + 'a {
    earthquakes
        .iter()
        .filter(move |quake| quake.magnitude >= min_magnitude)
        .filter(move |quake| {
            quake.center().haversine_distance_to(center).meters() <= max_distance_meters
        })
}
