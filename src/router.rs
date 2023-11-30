use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub fn build_router() -> Router {
	Router::new()
		.route("/", get(root_endpoint))
		.route("/-1/error", get(day_minusone))
}

pub async fn root_endpoint() -> impl IntoResponse {
	(
		StatusCode::OK,
		"Merry Christmas to all, and to all a good hunt!",
	)
		.into_response()
}

pub async fn day_minusone() -> impl IntoResponse {
	(StatusCode::INTERNAL_SERVER_ERROR, "B-B-B-B-BONUS!!!").into_response()
}
