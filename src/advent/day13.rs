use axum::{extract::State, response::IntoResponse, Json};

use crate::advent::db_common::{db_orders, db_reset, DBState};

pub fn router(state: &DBState) -> axum::Router {
	let orders = axum::Router::new()
		.route("/", axum::routing::post(db_orders))
		.route("/total", axum::routing::get(day_13_total))
		.route("/popular", axum::routing::get(day_13_popular))
		.with_state(state.clone());

	axum::Router::new()
		.route("/sql", axum::routing::get(day_13_sql))
		.route("/reset", axum::routing::post(db_reset))
		.nest("/orders", orders)
		.with_state(state.clone())
}

async fn day_13_sql(State(state): State<DBState>) -> impl IntoResponse {
	let out = sqlx::query!("SELECT 20231213 number")
		.fetch_one(&state.pool)
		.await
		.expect("Could not process query: test_number")
		.number
		.expect("Could not parse result");

	(axum::http::StatusCode::OK, format!("{out}"))
}

async fn day_13_total(State(state): State<DBState>) -> impl IntoResponse {
	let out = sqlx::query!("SELECT quantity FROM orders")
		.fetch_all(&state.pool)
		.await
		.expect("Could not process query: total_quantity")
		.iter()
		.map(|d| d.quantity.unwrap_or_default())
		.sum::<i32>();

	(
		axum::http::StatusCode::OK,
		Json(serde_json::json!({"total":out})),
	)
}

async fn day_13_popular(State(state): State<DBState>) -> impl IntoResponse {
	let out =
		sqlx::query!("SELECT SUM(quantity) AS quantity, gift_name FROM orders GROUP BY gift_name")
			.fetch_all(&state.pool)
			.await
			.expect("Could not process query: total_by_name")
			.into_iter()
			.max_by_key(|i| i.quantity);

	match out {
		Some(r) => (
			axum::http::StatusCode::OK,
			Json(serde_json::json!({"popular":r.gift_name.unwrap()})),
		),
		None => (
			axum::http::StatusCode::OK,
			Json(serde_json::json!({"popular":null})),
		),
	}
}
