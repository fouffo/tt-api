mod models;

use axum::{
    Router,
    response::{AppendHeaders, IntoResponse},
    routing::get,
};

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

async fn api_handler() -> impl IntoResponse {
    (
        AppendHeaders([("content-type", "application/json")]),
        "{\"test\":\"ok\"}",
    )
}
