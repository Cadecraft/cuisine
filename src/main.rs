use axum::{
    Router,
    extract::{Multipart, Query, State},
    http::{HeaderValue, Method, StatusCode},
    routing::{get, post},
};
use serde::Deserialize;
use std::collections::hash_map::HashMap;
use std::env;
use std::sync::{Arc, Mutex, MutexGuard};
use tower_http::cors::{Any, CorsLayer};

use cuisine::auth;
mod kv;

#[derive(Clone)]
struct AppState {
    cache: Arc<Mutex<HashMap<String, String>>>,
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    assert_env();

    let state = AppState {
        cache: Arc::new(Mutex::new(HashMap::new())),
    };

    let allowed_origins: Vec<HeaderValue> = env::var("ALLOWED_ORIGINS")
        .unwrap_or(String::new())
        .split(',')
        .map(|s| s.parse().expect("An allowed origin could not be parsed"))
        .collect();

    let app = Router::new()
        .route("/data", get(get_data))
        .route("/data", post(post_data))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_methods([Method::GET, Method::PUT])
                .allow_origin(allowed_origins),
        );

    let addr = format!(
        "{}:{}",
        env::var("HOST").unwrap(),
        env::var("PORT").unwrap()
    );
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct GetData {
    key: String,
}

async fn get_data(State(state): State<AppState>, query: Query<GetData>) -> String {
    let key = query.0.key;
    let mut cache = state.cache.lock().expect("Mutex was poisoned");
    match cache.get(&key) {
        Some(cached) => cached.clone(),
        None => {
            let new_val = kv::get_value(&key).unwrap_or(String::new());
            put_in_cache(&mut cache, key, new_val.clone());
            new_val
        }
    }
}

struct PutData {
    password: String,
    key: String,
    value: String,
}

async fn parse_post_multipart(multipart: &mut Multipart) -> Option<PutData> {
    let mut res = PutData {
        password: String::new(),
        key: String::new(),
        value: String::new(),
    };
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        // TODO: check performance with large files
        let data = field.bytes().await.unwrap();
        let string_data = String::from_utf8(data.to_vec()).ok()?;

        match name.as_str() {
            "password" => {
                res.password = string_data;
            }
            "key" => {
                res.key = string_data;
            }
            "value" => {
                res.value = string_data;
            }
            _ => {
                return None;
            }
        };
    }
    Some(res)
}

// TODO: test and update documentation
async fn post_data(State(state): State<AppState>, mut multipart: Multipart) -> StatusCode {
    // Use multipart (as opposed to JSON) for performance with large files
    let payload = match parse_post_multipart(&mut multipart).await {
        Some(p) => p,
        None => return StatusCode::BAD_REQUEST,
    };

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
