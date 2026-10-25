use axum::{
    extract::{Query, State},
    response::{AppendHeaders, IntoResponse, Json},
};
use http::StatusCode;
use chrono::{DateTime, Utc};
use std::time::SystemTime;
use serde_json::json;
use std::{collections::HashMap};
use std::sync::Arc;

use crate::utils::{get_closest_stop, get_next_runs};
use crate::AppState;

pub async fn api_handler(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>, // This is the app state being passed. It stores all stops and routes
) -> impl IntoResponse {
    let mut lat = 46.0719f32; // These are the coordinates of
    let mut lon = 11.1196f32; // the Trento train station
    let type_var: &str; // "type" is a keyword in Rust
    let current_time: DateTime<Utc> = SystemTime::now().into();
    let mut current_time = current_time.to_rfc3339();
    current_time.truncate("1970-01-01T00:00:00".len());

    //checks the latitude (default Stazione)
    if let Some(val) = params.get("lat") {
        lat = match val.parse::<f32>() {
            Ok(val) => val,
            Err(_) => return (
                StatusCode::BAD_REQUEST,
                AppendHeaders([("content-type", "application/json")]),
                Json(json!("{\"error\":\"latitude could not be parsed.\"}")),
            )
        }
    }

    //checks the longitude (default Stazione)
    if let Some(val) = params.get("lon") {
        lon = match val.parse::<f32>() {
            Ok(val) => val,
            Err(_) => return (
                StatusCode::BAD_REQUEST,
                AppendHeaders([("content-type", "application/json")]),
                Json(json!("{\"error\":\"latitude could not be parsed.\"}")),
            )
        }
    }

    if let Some(val) = params.get("type") {
        type_var = val;
    } else {
        type_var = "U" // The default bus type is "urbano"
    }

    if let Some(val) = params.get("time") {
        current_time = val.to_string();
    }

    let var_name = get_closest_stop(lat, lon, state.all_stops.clone()); // TODO: rename this

    let next_runs = get_next_runs(type_var, &current_time, var_name.stop_id, &state.all_routes).await;

    (
        StatusCode::OK,
        AppendHeaders([("content-type", "application/json")]),
        Json(json!(next_runs)),
    )
}

pub async fn stops_handler(
    State(state): State<Arc<AppState>>, // This is the app state being passed. It stores all stops and routes
) -> impl IntoResponse {
    (
        StatusCode::OK,
        AppendHeaders([("content-type", "application/json")]),
        Json(json!(&state.all_stops)),
    )
}

pub async fn routes_handler(
    State(state): State<Arc<AppState>>, // This is the app state being passed. It stores all stops and routes
) -> impl IntoResponse {
    (
        StatusCode::OK,
        AppendHeaders([("content-type", "application/json")]),
        Json(json!(&&state.all_routes)),
    )
}
