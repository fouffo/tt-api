use crate::models::{NextRunsList, Stop};

pub fn get_closest_stop(lat_num: f32, lon_num: f32, stops: Vec<Stop>) -> NextRunsList {
    let mut stop_lat: f32;
    let mut stop_lon: f32;
    let mut dist_min = f32::INFINITY;
    let mut dist: f32;
    let mut next_run = NextRunsList {
        stop_id: u32::MAX,
        stop_name: String::new(),
        next_trips: vec![],
    };

    for stop in stops {
        stop_lat = stop.lat;
        stop_lon = stop.lon;
        dist = distance(stop_lat, stop_lon, lat_num, lon_num);
        if dist < dist_min {
            dist_min = dist;
            next_run.stop_id = stop.id;
            next_run.stop_name = stop.name;
        }
    }
    next_run
}

//calculates the distance between the position coords annd the stop coords
fn distance(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f32 {
    let lat1 = lat1 * std::f32::consts::PI / 180.0;
    let lon1 = lon1 * std::f32::consts::PI / 180.0;
    let lat2 = lat2 * std::f32::consts::PI / 180.0;
    let lon2 = lon2 * std::f32::consts::PI / 180.0;

    let delta_lat = lat1 - lat2;
    let delta_lon = lon1 - lon2;

    12742000.0 // This magic number is the diameter of the earth in meters.
        * (
                (delta_lat / 2.0).sin().powi(2) +
                lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2)
            )
            .sqrt()
            .asin()
}
