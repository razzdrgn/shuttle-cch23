use axum::{
	extract::Path,
	response::IntoResponse
};

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/1/*serial", axum::routing::get(day_one))
}

async fn day_one(Path(serial): Path<String>) -> impl IntoResponse {
	let v: Vec<&str> = serial.split_terminator('/').collect();
	let mut result: i32 = 0;
	for i in v.iter() {
		result = result ^ i.parse::<i32>().unwrap_or(0);
	}
	(axum::http::StatusCode::OK, result.pow(3).to_string()).into_response()
}