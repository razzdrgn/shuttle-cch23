use std::collections::HashMap;

use axum::{extract::Query, response::IntoResponse, Json};
use serde_json::{json, Value};

pub fn router() -> axum::Router {
	axum::Router::new().route("/", axum::routing::post(day_five))
}

async fn day_five(
	Query(q): Query<HashMap<String, usize>>,
	Json(b): Json<Value>,
) -> impl IntoResponse {
	let vals: Vec<String> = serde_json::from_value(b).expect("failed deserialization");
	let len = vals.len();

	let off = *q.get("offset").unwrap_or(&0);
	let lim = *q.get("limit").unwrap_or(&len);
	let spl = *q.get("split").unwrap_or(&0);

	let mut outvals: Vec<String> = Vec::new();
	let mut out: Vec<Vec<String>> = Vec::new();

	for i in 0..lim {
		if vals.get(i + off).is_some() {
			outvals.push(vals.get(i + off).unwrap().to_string());
		}
	}

	if spl != 0 {
		for _ in 0..(outvals.len() / spl) {
			if outvals.len() <= spl {
				break;
			}
			let temp = outvals.split_off(spl);
			out.push(outvals);
			outvals = temp;
		}
	}

	if !outvals.is_empty() {
		out.push(outvals);
	}

	if out.len() == 1 {
		return (
			axum::http::StatusCode::OK,
			Json(json!(out.into_iter().flatten().collect::<Vec<String>>())),
		);
	}

	(axum::http::StatusCode::OK, Json(json!(out)))
}
