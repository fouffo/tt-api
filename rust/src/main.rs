mod models;
mod utils;
mod handlers;

use axum::{
    Router,
    routing::get,
};
use std::sync::Arc;

use crate::utils::{get_route_list, get_stop_list};

// This struct represents the app state which can be read by all endpoints.
#[derive(Clone)]
struct AppState {
    all_stops: Vec<models::mitt::Stop>,
    all_routes: Vec<models::mitt::Route>,
}

#[tokio::main]
async fn main() {
    // The app state gets initialized with the full list of stops and routes when the server starts
    let shared_state = AppState {
        all_stops: get_stop_list().await,
        all_routes: get_route_list().await,
    };

    let app = Router::new()
        .route("/api", get(handlers::api_handler))
        .route("/stops", get(handlers::stops_handler))
        .route("/routes", get(handlers::routes_handler))
        .with_state(Arc::new(shared_state)); // The shared state instance we created is declared as the one to be used by Axum. Arc is used for thread safety (I think)

    match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(list) => axum::serve(list, app)
            .await
            .expect("The server could not be started for an unknown reason."),
        Err(e) => println!(
            "The server could not be started for the following reason : {}",
            e.to_string()
        ),
    }
}
