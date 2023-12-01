use axum::response::IntoResponse;

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/", axum::routing::get(root_endpoint))
		.route("/-1", axum::routing::get(day_zero))
}

async fn root_endpoint() -> impl IntoResponse {
	(
		axum::http::StatusCode::OK,
		"Merry Christmas to all, and to all a good hunt!",
	)
		.into_response()
}

pub async fn day_zero() -> impl IntoResponse {
	(
		axum::http::StatusCode::INTERNAL_SERVER_ERROR,
		"B-B-B-B-BONUS!!!",
	)
		.into_response()
}
