use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NextRunsList {
    // TODO: rename
    pub stop_id: u16,
    pub stop_name: String,
    pub next_trips: Vec<i32>, // TODO: replace i32 with the proper type
}

#[derive(Deserialize, Serialize)]
pub struct Trip {
    pub route_color: String,
    pub route_short_name: String,
    pub route_long_name: String,
    pub delay: String, // TODO: Check if this is actually a string or something else
    pub arrival_time: String, // TODO: Check this too
}
