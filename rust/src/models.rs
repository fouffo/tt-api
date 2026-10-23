use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NextRunsList {
    pub stop_id: u32,
    pub stop_name: String,
    pub next_trips: Vec<i32>, // TODO: replace i32 with the proper type
}

#[derive(Deserialize, Serialize)]
pub struct Stop {
    pub id: u32,
    pub name: String,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Deserialize, Serialize)]
pub struct Route {
    pub id: u32,
    pub short_name: String,
    pub long_name: String,
    pub color: String,
}
