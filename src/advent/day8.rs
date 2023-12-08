use axum::{extract::Path, response::IntoResponse};
use num::Float;

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/8", axum::routing::get(axum::http::StatusCode::OK))
		.route("/8/weight/:dex_num", axum::routing::get(day_eight_weight))
		.route("/8/drop/:dex_num", axum::routing::get(day_eight_drop))
}

async fn query_mon(dex_num: String) -> serde_json::Value {
	serde_json::from_str(
		&reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{dex_num}"))
			.await
			.expect("Request could not complete")
			.text()
			.await
			.expect("Could not parse bytestream"),
	)
	.expect("Could not deserialize JSON response")
}

async fn day_eight_weight(Path(dex_num): Path<String>) -> impl IntoResponse {
	let resp: serde_json::Value = query_mon(dex_num).await;

	(
		axum::http::StatusCode::OK,
		format!(
			"{}",
			resp.get("weight")
				.expect("Could not parse response object (BAD READ)")
				.as_u64()
				.expect("Could not parse response object (BAD CAST)")
				/ 10
		),
	)
}

#[allow(clippy::cast_precision_loss)] // working with small as hell numbers. don't worry abt precision here
async fn day_eight_drop(Path(dex_num): Path<String>) -> impl IntoResponse {
	let resp: serde_json::Value = query_mon(dex_num).await;

	let weight: u64 = resp
		.get("weight")
		.expect("Could not parse response object (BAD READ)")
		.as_u64()
		.expect("Could not parse response object (BAD CAST)")
		/ 10;

	let vel: f64 = Float::sqrt(20_f64 / 9.825) * 9.825;

	(
		axum::http::StatusCode::OK,
		format!("{}", weight as f64 * vel),
	)
}
