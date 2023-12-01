pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/11", axum::routing::get(axum::http::StatusCode::OK))
}