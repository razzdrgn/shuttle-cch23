use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::Value;
use sqlx::PgPool;
use tracing::info;

#[derive(Clone)]
pub(crate) struct DBState {
	pub(crate) pool: sqlx::PgPool,
}

impl DBState {
	pub async fn init(pool: PgPool) -> Result<DBState, shuttle_runtime::CustomError> {
		sqlx::migrate!()
			.run(&pool)
			.await
			.map_err(shuttle_runtime::CustomError::new)?;

		Ok(DBState { pool })
	}
}

#[derive(Deserialize)]
struct Order {
	id: i32,
	region_id: i32,
	gift_name: String,
	quantity: i32,
}

#[derive(Deserialize)]
struct Region {
	id: i32,
	name: String,
}

pub async fn db_reset(State(state): State<DBState>) -> impl IntoResponse {
	sqlx::query!("DROP TABLE IF EXISTS orders")
		.execute(&state.pool)
		.await
		.expect("Could not reset database: Table does not exist- skipping drop");

	sqlx::query!("DROP TABLE IF EXISTS regions")
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

	sqlx::query!(
		"CREATE TABLE regions (
		id INT PRIMARY KEY,
		name VARCHAR(50)
	  )"
	)
	.execute(&state.pool)
	.await
	.expect("Could not reset database: Could not initialize table schema");

	axum::http::StatusCode::OK
}

pub async fn db_orders(State(state): State<DBState>, Json(body): Json<Value>) -> impl IntoResponse {
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

pub async fn db_regions(
	State(state): State<DBState>,
	Json(body): Json<Value>,
) -> impl IntoResponse {
	info!("Request Body: {body}");
	let inputs: Vec<Region> = serde_json::from_value(body).expect("Could not parse inputs");

	for i in inputs {
		sqlx::query!(
			"INSERT INTO regions (id, name) VALUES ($1, $2)",
			i.id,
			i.name
		)
		.execute(&state.pool)
		.await
		.unwrap_or_else(|_| panic!("Could not insert item {{{}, {}}}", i.id, i.name));
	}

	axum::http::StatusCode::OK
}
