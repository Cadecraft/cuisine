use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, put},
};
use serde::Deserialize;

mod kv;

#[tokio::main]
async fn main() {
    // TODO: get env

    let app = Router::new()
        .route("/data", get(get_data))
        .route("/data", put(put_data));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_data(Json(payload): Json<GetData>) -> String {
    // TODO: caching?
    kv::get_value(&payload.key).unwrap_or(String::new())
}

async fn put_data(Json(payload): Json<PutData>) -> StatusCode {
    // TODO: auth
    // TODO: impl
    StatusCode::UNAUTHORIZED
}

#[derive(Deserialize)]
struct GetData {
    key: String,
}

#[derive(Deserialize)]
struct PutData {
    key: String,
    value: String,
    auth: String,
}
