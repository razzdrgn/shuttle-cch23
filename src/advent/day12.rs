use axum::{
	extract::{Path, State},
	response::IntoResponse,
	Json,
};
use chrono::prelude::*;
use serde::Serialize;
use serde_json::Value;
use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
	time::Instant,
};
use tracing::info;
use ulid::Ulid;
use uuid::Uuid;

pub fn router() -> axum::Router {
	let state = Arc::new(RwLock::new(TimeState {
		map: HashMap::new(),
	}));

	axum::Router::new()
		.route("/save/:key", axum::routing::post(save_key))
		.route("/load/:key", axum::routing::get(fetch_key))
		.route("/ulids", axum::routing::post(day_twelve_ulids))
		.route("/ulids/:day", axum::routing::post(day_twelve_lsb))
		.with_state(state)
}

struct TimeState {
	map: HashMap<String, Instant>,
}

#[derive(Serialize)]
struct LsbOutput {
	#[serde(rename(serialize = "christmas eve"))]
	christmas_eve: usize,
	weekday: usize,
	#[serde(rename(serialize = "in the future"))]
	future: usize,
	#[serde(rename(serialize = "LSB is 1"))]
	lsb: usize,
}

impl LsbOutput {
	fn new(i: &[Ulid], day: u32) -> LsbOutput {
		let dates = i
			.iter()
			.map(Ulid::datetime)
			.map(DateTime::<Utc>::from);

		let christmas_eve = dates.clone().filter(|d| d.month() == 12 && d.day() == 24).count();
		let weekday = dates.clone().filter(|d| d.weekday().num_days_from_monday() == day).count();
		let future = dates.filter(|d| d.gt(&Utc::now())).count();
		let lsb = i.iter().filter(|u| u.0 & 1 == 1).count();

		LsbOutput {
			christmas_eve,
			weekday,
			future,
			lsb
		}
	}
}

async fn save_key(
	State(state): State<Arc<RwLock<TimeState>>>,
	Path(key): Path<String>,
) -> impl IntoResponse {
	state
		.write()
		.expect("Could not access shared state as writeable")
		.map
		.insert(key, Instant::now());
	axum::http::StatusCode::OK
}

async fn fetch_key(
	State(state): State<Arc<RwLock<TimeState>>>,
	Path(key): Path<String>,
) -> impl IntoResponse {
	let state = state
		.read()
		.expect("Could not access shared state as readble");
	let earlier = state.map.get(&key).unwrap();
	(
		axum::http::StatusCode::OK,
		format!("{}", Instant::now().duration_since(*earlier).as_secs()),
	)
}

async fn day_twelve_ulids(Json(body): Json<Value>) -> impl IntoResponse {
	info!("Request Body: {body}");
	let inputs: Vec<Ulid> = serde_json::from_value(body).expect("Could not deserialize input");
	let outputs: Vec<Uuid> = inputs.into_iter().rev().map(Uuid::from).collect();
	(axum::http::StatusCode::OK, Json(outputs))
}

async fn day_twelve_lsb(Path(day): Path<u32>, Json(body): Json<Value>) -> impl IntoResponse {
	if day > 6 {
		return axum::http::StatusCode::BAD_REQUEST.into_response();
	};
	info!("Request Body: {body}");
	let inputs: Vec<Ulid> = serde_json::from_value(body).expect("Could not deserialize input");

	(axum::http::StatusCode::OK, Json(LsbOutput::new(&inputs, day))).into_response()
}
