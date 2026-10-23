use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NextRunsList {
    pub stop_id: i32,
    pub stop_name: String,
    pub next_trips: Vec<i32>, // TODO: replace i32 with the proper type
}
