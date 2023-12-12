use axum::{extract::{State, Path}, response::IntoResponse, Json};
use serde_json::Value;
use tracing::info;
use std::{sync::{Arc, RwLock}, time::Instant, collections::HashMap};
use uuid::Uuid;
use ulid::Ulid;

pub fn router() -> axum::Router {
	let state = Arc::new(RwLock::new(TimeState {map: HashMap::new()}));

	axum::Router::new()
		.route("/12", axum::routing::get(axum::http::StatusCode::OK))
		.route("/12/save/:key", axum::routing::post(save_key))
		.route("/12/load/:key", axum::routing::get(fetch_key))
		.route("/12/ulids", axum::routing::post(day_twelve_ulids))
		.route("/12/ulids/:day", axum::routing::post(axum::http::StatusCode::NOT_IMPLEMENTED))
		.with_state(state)
}

struct TimeState {
	map: HashMap<String, Instant>
}

async fn save_key(State(state): State<Arc<RwLock<TimeState>>>, Path(key): Path<String>) -> impl IntoResponse {
	state.write().expect("Could not access shared state as writeable").map.insert(key, Instant::now());
	axum::http::StatusCode::OK
}

async fn fetch_key(State(state): State<Arc<RwLock<TimeState>>>, Path(key): Path<String>) -> impl IntoResponse {
	let state = state.read().expect("Could not access shared state as readble");
	let earlier = state.map.get(&key).unwrap();
	(axum::http::StatusCode::OK, format!("{}", Instant::now().duration_since(*earlier).as_secs()))
}

async fn day_twelve_ulids(Json(body): Json<Value>) -> impl IntoResponse {
	info!("Request Body: {body}");
	let inputs: Vec<Ulid> = serde_json::from_value(body).expect("Could not deserialize input");
	let outputs: Vec<Uuid> = inputs.iter().rev().map(|u| Uuid::from(*u)).collect();
	(axum::http::StatusCode::OK, Json(outputs))
}