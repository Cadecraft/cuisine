use axum::{
    routing::{get, put},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/data", get(get_data))
        .route("/data", put(put_data));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_data(Json(payload): Json<GetData>) -> String {
    format!("Hello world {}", payload.key)
}

async fn put_data(Json(payload): Json<PutData>) -> StatusCode {
    // TODO: impl
    StatusCode::UNAUTHORIZED
}

#[derive(Deserialize)]
struct GetData {
    key: String
}

#[derive(Deserialize)]
struct PutData {
    key: String,
    value: String,
    auth: String,
}
