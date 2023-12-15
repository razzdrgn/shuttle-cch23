use axum::{response::IntoResponse, Json};
use fancy_regex::Regex;
use serde_json::{json, Value};
use tracing::info;

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/nice", axum::routing::post(day_15_nice))
		.route(
			"/game",
			axum::routing::post(axum::http::StatusCode::NOT_IMPLEMENTED),
		)
}

async fn day_15_nice(body: String) -> impl IntoResponse {
	info!("Request body: {body}");

	let input: Result<Value, _> = serde_json::from_str(&body);

	if input.is_err() {
		return axum::http::StatusCode::BAD_REQUEST.into_response();
	}

	let input: String = serde_json::from_value(input.unwrap().clone()).unwrap();

	let vowels = Regex::new(r"[aeiouy]").unwrap();
	let repeated = Regex::new(r"(.)\1").unwrap();
	let bad_patterns = Regex::new(r"ab|cd|pq|xy").unwrap();

	let vowels_test = vowels
		.find_iter(&input)
		.count() >= 3;

	info!("Contains at least three vowels: {vowels_test}");
	let repeated_test = repeated.find(&input).unwrap().is_some();
	info!("Contains at least one repeated letter: {repeated_test}");
	let bad_patterns_test = bad_patterns.find(&input).unwrap().is_none();
	info!("Does not contain 'ab', 'cd', 'pq', or 'xy': {bad_patterns_test}");
	let out = if vowels_test && repeated_test && bad_patterns_test { "nice" } else { "naughty" };

	(axum::http::StatusCode::OK, Json(json!({"result": out}))).into_response()
}
