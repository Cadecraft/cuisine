use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, put},
};
use serde::Deserialize;
use std::collections::hash_map::HashMap;
use std::env;
use std::sync::{Arc, Mutex, MutexGuard};

use cuisine::auth;
mod kv;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env must exist");
    assert_env();

    let state = AppState {
        cache: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/data", get(get_data))
        .route("/data", put(put_data))
        .with_state(state);

    let addr = format!(
        "{}:{}",
        env::var("HOST").unwrap(),
        env::var("PORT").unwrap()
    );
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn get_data(State(state): State<AppState>, Json(payload): Json<GetData>) -> String {
    let mut cache = state.cache.lock().expect("Mutex was poisoned");
    match cache.get(&payload.key) {
        Some(cached) => cached.clone(),
        None => {
            let new_val = kv::get_value(&payload.key).unwrap_or(String::new());
            put_in_cache(&mut cache, payload.key, new_val.clone());
            new_val
        }
    }
}

async fn put_data(State(state): State<AppState>, Json(payload): Json<PutData>) -> StatusCode {
    if !auth::is_authorized(&payload.password) {
        return StatusCode::UNAUTHORIZED;
    }
    match kv::put_value(&payload.key, &payload.value) {
        Ok(_) => {
            // Update cache
            let mut cache = state.cache.lock().expect("Mutex was poisoned");
            put_in_cache(&mut cache, payload.key, payload.value);
            StatusCode::OK
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn put_in_cache(cache: &mut MutexGuard<'_, HashMap<String, String>>, key: String, value: String) {
    if value.is_empty() {
        cache.remove(&key);
    } else {
        cache.insert(key, value);
    }
}

/// Ensure the proper environment variables are provided
fn assert_env() {
    let needed = ["ADMIN_AUTH", "HOST", "PORT", "KV_PATH"];
    for var in needed {
        env::var(var).expect(&format!("{} must be provided", var));
    }
    if !auth::is_valid_argon2(&env::var("ADMIN_AUTH").unwrap()) {
        panic!("ADMIN_AUTH must be a valid argon2 hash");
    }
}

#[derive(Deserialize)]
struct GetData {
    key: String,
}

#[derive(Deserialize)]
struct PutData {
    key: String,
    value: String,
    password: String,
}

#[derive(Clone)]
struct AppState {
    cache: Arc<Mutex<HashMap<String, String>>>,
}
