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
use std::sync::Arc;

use crate::utils::{get_closest_stop, get_next_runs, get_route_list, get_stop_list};

#[derive(Clone)]
struct AppState {
    all_stops: Vec<models::mitt::Stop>,
    all_routes: Vec<models::mitt::Route>,
}

#[tokio::main]
async fn main() {
    let shared_state = AppState {
        all_stops: get_stop_list().await,
        all_routes: get_route_list().await,
    };

    let app = Router::new()
        .route("/api", get(api_handler))
        .with_state(Arc::new(shared_state));

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
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let mut lat = 46.0719f32; // These are the coordinates of
    let mut lon = 11.1196f32; // the Trento train station
    let type_var: &str; // "type" is a keyword in Rust
    let current_time;

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

    let var_name = get_closest_stop(lat, lon, state.all_stops.clone()); // TODO: rename this

    let next_runs = get_next_runs(type_var, &current_time, var_name.stop_id, &state.all_routes).await;

    (
        AppendHeaders([("content-type", "application/json")]),
        Json(json!(next_runs)),
    )
}
