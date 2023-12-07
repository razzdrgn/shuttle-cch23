use std::collections::HashMap;

use axum::{response::IntoResponse, Json};
use axum_extra::TypedHeader;
use base64::{engine::general_purpose, Engine as _};
use headers::Cookie;
use serde::{Deserialize, Serialize};

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/7", axum::routing::get(axum::http::StatusCode::OK))
		.route("/7/decode", axum::routing::get(day_seven_decode))
		.route("/7/bake", axum::routing::get(day_seven_bake))
}

#[derive(Serialize, Deserialize)]
struct BakeInput {
	recipe: HashMap<String, usize>,
	pantry: HashMap<String, usize>,
}

#[derive(Serialize, Deserialize)]
struct BakeOutput {
	cookies: usize,
	pantry: HashMap<String, usize>,
}

impl BakeInput {
	fn bake(self) -> BakeOutput {
		let cookies = self
			.pantry
			.iter()
			.map(|(x, y)| {
				(
					x.clone(),
					usize::checked_div(*y, *self.recipe.get(x).unwrap_or(&0)).unwrap_or(0),
				)
			})
			.reduce(|x, y| if x.1 < y.1 { x } else { y })
			.expect("Could not find minimum cookies")
			.1;
		
		BakeOutput {
			cookies,
			pantry: self
				.pantry
				.iter()
				.map(|(x, y)| (x.clone(), y - (self.recipe.get(x).unwrap_or(&0) * cookies)))
				.collect(),
		}
	}
}

fn decode(input: &str) -> String {
	String::from_utf8(
		general_purpose::STANDARD
			.decode(input)
			.expect("Could not decode recipe"),
	)
	.expect("Could not parse into string")
}

async fn day_seven_decode(TypedHeader(cookie): TypedHeader<Cookie>) -> impl IntoResponse {
	(
		axum::http::StatusCode::OK,
		decode(cookie.get("recipe").expect("Could not parse recipe")),
	)
}

async fn day_seven_bake(TypedHeader(cookie): TypedHeader<Cookie>) -> impl IntoResponse {
	let decoded = decode(cookie.get("recipe").expect("Could not parse recipe"));

	let input: BakeInput = serde_json::from_str(&decoded).expect("Could not deserialize JSON");

	(axum::http::StatusCode::OK, Json(input.bake()))
}
