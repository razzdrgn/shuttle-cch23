use axum::{
	extract::{Path, State},
	response::IntoResponse,
	Json,
};
use serde_json::{json, Value};
use sqlx::Row;

use crate::advent::db_common::{db_orders, db_regions, db_reset, DBState};

pub fn router(state: &DBState) -> axum::Router {
	let regions = axum::Router::new()
		.route("/", axum::routing::post(db_regions))
		.route("/total", axum::routing::get(day_18_total))
		.route("/top_list/:top", axum::routing::get(day_18_top))
		.with_state(state.clone());

	axum::Router::new()
		.route("/reset", axum::routing::post(db_reset))
		.route("/orders", axum::routing::post(db_orders))
		.nest("/regions", regions)
		.with_state(state.clone())
}

async fn day_18_total(State(state): State<DBState>) -> impl IntoResponse {
	let out: Vec<Value> = sqlx::query!("SELECT regions.name, SUM(orders.quantity) as total FROM orders INNER JOIN regions ON orders.region_id = regions.id GROUP BY regions.name ORDER BY regions.name")
		.fetch_all(&state.pool)
		.await
		.expect("Could not process query: total_region")
		.iter()
		.map(|d| json!({"region":d.name.clone().unwrap_or_default(),"total":d.total.unwrap_or_default()}))
		.collect();

	(axum::http::StatusCode::OK, Json(out)).into_response()
}

async fn day_18_top(State(state): State<DBState>, Path(top): Path<i32>) -> impl IntoResponse {
	let regions = sqlx::query!("SELECT id, name FROM regions ORDER BY name")
		.fetch_all(&state.pool)
		.await
		.expect("Could not process query: fetch_regions");

	let mut out: Vec<Value> = Vec::new();

	for r in regions {
		let q: Vec<String> = sqlx::query(
			format!("SELECT gift_name FROM orders WHERE region_id = {} GROUP BY gift_name ORDER BY SUM(quantity) DESC LIMIT {}", r.id, top)
			.as_str())
		.fetch_all(&state.pool)
		.await
		.unwrap_or_else(|_| panic!("Could not process query: fetch_region_{0}", r.name.clone().unwrap_or_default()))
		.iter()
		.map(|s| s.get::<String, &str>("gift_name"))
		.collect();

		out.push(json!({"region":r.name.unwrap(),"top_gifts":q}));
	}

	(axum::http::StatusCode::OK, Json(out)).into_response()
}
