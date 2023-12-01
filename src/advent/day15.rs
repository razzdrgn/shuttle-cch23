pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/15", axum::routing::get(axum::http::StatusCode::OK))
}