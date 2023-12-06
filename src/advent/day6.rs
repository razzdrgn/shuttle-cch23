use axum::{response::IntoResponse, Json};
use fancy_regex::Regex;

pub fn router() -> axum::Router {
	axum::Router::new().route("/6", axum::routing::post(day_six))
}

#[derive(serde::Serialize)]
struct Response {
	elf: usize,
	elf_on_a_shelf: usize,
	shelf_with_no_elf_on_it: usize,
}

impl Response {
	fn new(text: &str) -> Response {
		let elf = Regex::new("elf")
			.expect("Could not make regex")
			.captures_iter(text)
			.count();
		let elf_on_a_shelf = Regex::new("elf on a shelf")
			.expect("Could not make regex")
			.captures_iter(text)
			.count();
		let shelf_with_no_elf_on_it = Regex::new("(?<!elf on a )shelf")
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
