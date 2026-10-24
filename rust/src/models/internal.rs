use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NextRunsList {
    // TODO: rename
    pub stop_id: u32,
    pub stop_name: String,
    pub next_trips: Vec<i32>, // TODO: replace i32 with the proper type
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Stop {
    pub id: u32,
    pub name: String,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Route {
    pub id: u32,
    pub short_name: String,
    pub long_name: String,
    pub color: String,
}

#[derive(Deserialize, Serialize)]
pub struct Trip {
    pub route_color: String,
    pub route_short_name: String,
    pub route_long_name: String,
    pub delay: String, // TODO: Check if this is actually a string or something else
    pub arrival_time: String, // TODO: Check this too
}
