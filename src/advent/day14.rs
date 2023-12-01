pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/14", axum::routing::get(axum::http::StatusCode::OK))
}