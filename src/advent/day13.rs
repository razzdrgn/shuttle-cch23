use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::Value;
use tracing::info;

use crate::advent::DBState;

pub fn router(state: &DBState) -> axum::Router {
	let orders = axum::Router::new()
		.route("/", axum::routing::post(day_13_orders))
		.route("/total", axum::routing::get(day_13_total))
		.route("/popular", axum::routing::get(day_13_popular))
		.with_state(state.clone());

	axum::Router::new()
		.route("/sql", axum::routing::get(day_13_sql))
		.route("/reset", axum::routing::post(day_13_reset))
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

async fn day_13_reset(State(state): State<DBState>) -> impl IntoResponse {
	sqlx::query!("DROP TABLE IF EXISTS orders")
		.execute(&state.pool)
		.await
		.expect("Could not reset database: Table does not exist- skipping drop");

	sqlx::query!(
		"CREATE TABLE orders (
		id INT PRIMARY KEY,
		region_id INT,
		gift_name VARCHAR(50),
		quantity INT
	  )"
	)
	.execute(&state.pool)
	.await
	.expect("Could not reset database: Could not initialize table schema");

	axum::http::StatusCode::OK
}

#[derive(Deserialize)]
struct Order {
	id: i32,
	region_id: i32,
	gift_name: String,
	quantity: i32,
}

async fn day_13_orders(State(state): State<DBState>, Json(body): Json<Value>) -> impl IntoResponse {
	info!("Request Body: {body}");
	let inputs: Vec<Order> = serde_json::from_value(body).expect("Could not parse inputs");

	for i in inputs {
		sqlx::query!(
			"INSERT INTO orders (id, region_id, gift_name, quantity) VALUES ($1, $2, $3, $4)",
			i.id,
			i.region_id,
			i.gift_name,
			i.quantity
		)
		.execute(&state.pool)
		.await
		.unwrap_or_else(|_| {
			panic!(
				"Could not insert item {{{}, {}, {}, {}}}",
				i.id, i.region_id, i.gift_name, i.quantity
			)
		});
	}

	axum::http::StatusCode::OK
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
