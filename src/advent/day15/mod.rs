use axum::{response::IntoResponse, Json};
use fancy_regex::Regex;
use serde_json::{json, Value};
use tracing::info;

mod passwordgame;
use passwordgame::PasswordGame;

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/nice", axum::routing::post(day_15_nice))
		.route("/game", axum::routing::post(day_15_game))
}

fn process_input(input: &str) -> String {
	info!("Request body: {input}");

	match serde_json::from_str::<Value>(input) {
		Err(_) => String::new(),
		Ok(v) => match v.get("input") {
			None => String::new(),
			Some(v) => match serde_json::from_value::<String>(v.clone()) {
				Err(_) => String::new(),
				Ok(s) => s,
			},
		},
	}
}

async fn day_15_nice(body: String) -> impl IntoResponse {
	let input = process_input(&body);

	if input.is_empty() {
		return axum::http::StatusCode::BAD_REQUEST.into_response();
	}

	let vowels = Regex::new(r"[aeiouy]").unwrap();
	let repeated = Regex::new(r"([\D\W])\1").unwrap();
	let bad_patterns = Regex::new(r"ab|cd|pq|xy").unwrap();

	let vowels_test = vowels.find_iter(&input).count() >= 3;

	info!("Contains at least three vowels: {vowels_test}");
	let repeated_test = repeated.find(&input).unwrap().is_some();
	info!("Contains at least one repeated letter: {repeated_test}");
	let bad_patterns_test = bad_patterns.find(&input).unwrap().is_none();
	info!("Does not contain 'ab', 'cd', 'pq', or 'xy': {bad_patterns_test}");

	if vowels_test && repeated_test && bad_patterns_test {
		(axum::http::StatusCode::OK, Json(json!({"result": "nice"}))).into_response()
	} else {
		(
			axum::http::StatusCode::BAD_REQUEST,
			Json(json!({"result": "naughty"})),
		)
			.into_response()
	}
}

async fn day_15_game(body: String) -> impl IntoResponse {
	let input = process_input(&body);

	if input.is_empty() {
		return axum::http::StatusCode::BAD_REQUEST.into_response();
	}

	PasswordGame::new(&input).create_response().into_response()
}
