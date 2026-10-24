mod models;
mod utils;

use axum::{
    Router,
    extract::{Query, State},
    response::{AppendHeaders, IntoResponse, Json},
    routing::get,
};
use serde_json::json;
use std::collections::HashMap;

use crate::models::{Route, Stop};
use crate::utils::get_closest_stop;

const URL: &str = "https://app-tpl.tndigit.it/gtlservice";
const USR: &str = "mittmobile";
const PSW: &str = "ecGsp.RHB3";
const CLOSEST_STOPS_COUNT: u8 = 5;

#[derive(Clone)]
struct AppState {
    all_stops: Vec<Stop>,
    all_routes: Vec<Route>,
}

#[tokio::main]
async fn main() {
    let shared_state = AppState {
        all_stops: Vec::new(),
        all_routes: Vec::new(),
    };

    let app = Router::new()
        .route("/api", get(api_handler))
        .with_state(shared_state);

    match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(list) => axum::serve(list, app)
            .await
            .expect("The server could not be started for an unknown reason."),
        Err(e) => println!(
            "The server could not be started for the following reason : {:?}",
            e.to_string()
        ),
    }
}

async fn api_handler(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut lat = 46.0719f32; // These are the coordinates of
    let mut lon = 11.1196f32; // the Trento train station
    let mut type_var = ""; // "type" is a keyword in Rust
    let mut current_time;

    //checks the latitude (default Stazione)
    if params.contains_key("lat") {
        lat = params
            .get("lat")
            .expect("")
            .parse::<f32>()
            .expect("test")
            .into();
    }

    //checks the longitude (default Stazione)
    if params.contains_key("lon") {
        lon = params
            .get("lon")
            .expect("")
            .parse::<f32>()
            .expect("test")
            .into();
    }

    if params.contains_key("type") {
        type_var = params.get("type").unwrap();
    } else {
        type_var = "U";
    }

    if params.contains_key("time") {
        current_time = params.get("time").unwrap().to_string();
    } else {
        current_time = "2026-10-19T07:50:00Z".to_string(); //to be changed with the time of the server
    }

    let next_runs_list = get_closest_stop(lat, lon, state.all_stops);

    (
        AppendHeaders([("content-type", "application/json")]),
        Json(json!(models::NextRunsList {
            stop_id: u32::MAX,
            stop_name: params.get("stop").unwrap().into(),
            next_trips: vec![],
        })),
    )
}
