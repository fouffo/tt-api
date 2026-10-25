mod models;
mod utils;
mod handlers;

use axum::{
    Router,
    routing::get,
};
use std::{sync::Arc, thread, time};
use tokio::{task, sync::Mutex};

use crate::utils::{get_route_list, get_stop_list};

// This struct represents the app state which can be read by all endpoints.
#[derive(Clone, Debug)]
struct AppState {
    all_stops: Vec<models::mitt::Stop>,
    all_routes: Vec<models::mitt::Route>,
}

#[tokio::main]
async fn main() {
    // The app state gets initialized with the full list of stops and routes when the server starts
    let shared_state = Arc::new(Mutex::new(AppState {
        all_stops: get_stop_list().await,
        all_routes: get_route_list().await,
    }));

    let app = Router::new()
        .route("/api", get(handlers::api_handler))
        .route("/stops", get(handlers::stops_handler))
        .route("/routes", get(handlers::routes_handler))
        .with_state(shared_state.clone()); // The shared state instance we created is declared as the one to be used by Axum. Arc is used for thread safety (I think)

    task::spawn(async move {
        loop {
            let stops = get_stop_list().await;
            let routes = get_route_list().await;
            shared_state.lock().await.all_routes = routes;
            shared_state.lock().await.all_stops = stops;
            thread::sleep(time::Duration::from_hours(24)); // updates every day
        }
    });

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
