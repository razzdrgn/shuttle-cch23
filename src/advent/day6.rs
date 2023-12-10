use axum::{response::IntoResponse, Json};
use fancy_regex::Regex;

pub fn router() -> axum::Router {
	axum::Router::new().route("/6", axum::routing::post(day_six))
}

#[derive(serde::Serialize)]
struct Response {
	elf: usize,
	#[serde(rename(serialize = "elf on a shelf"))]
	elf_on_a_shelf: usize,
	#[serde(rename(serialize = "shelf with no elf on it"))]
	shelf_with_no_elf_on_it: usize,
}

impl Response {
	fn new(text: &str) -> Response {
		tracing::info!("{}", text);
		let elf = Regex::new("(?i)elf")
			.expect("Could not make regex")
			.captures_iter(text)
			.count();
		let elf_on_a_shelf = Regex::new("(?i)elf on a shelf")
			.expect("Could not make regex")
			.captures_iter(text)
			.count();
		let shelf_with_no_elf_on_it = Regex::new("(?i)(?<!elf on a )shelf")
			.expect("Could not make regex")
			.captures_iter(text)
			.count();
		Response {
			elf,
			elf_on_a_shelf,
			shelf_with_no_elf_on_it,
		}
	}
}

async fn day_six(body: String) -> impl IntoResponse {
	(axum::http::StatusCode::OK, Json(Response::new(&body)))
}
