mod models;

use axum::{
    Router,
    extract::Query,
    response::{AppendHeaders, IntoResponse, Json},
    routing::get,
};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api", get(api_handler));

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

async fn api_handler(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    (
        AppendHeaders([("content-type", "application/json")]),
        Json(json!(models::NextRunsList {
            stop_id: -1,
            stop_name: params.get("stop").unwrap().into(),
            next_trips: vec![],
        })),
    )
}
