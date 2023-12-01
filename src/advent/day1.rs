use axum::{extract::Path, response::IntoResponse};

pub fn router() -> axum::Router {
	axum::Router::new().route("/1/*serial", axum::routing::get(day_one))
}

async fn day_one(Path(serial): Path<String>) -> impl IntoResponse {
	(
		axum::http::StatusCode::OK,
		serial
			.split_terminator('/') // Split the path into individual strings
			.map(|i| i.parse::<i32>().unwrap_or(0)) // Parse the strings into signed ints (defaulting to 0 on fail)
			// Since we're trying to do bitwise XOR, we want the default fail case to be 0
			// This is due to the property that when XORing with 0, the original number is always preserved
			// So in case any parsing errors do occur, it won't affect the numeric result at all
			.reduce(|x, y| x ^ y) // Perform the bitwise xor operation
			.map(|o| o.pow(3)) // Perform the power operation on the result and convert back to string
			.unwrap()
			.to_string(),
	)
		.into_response()
}
